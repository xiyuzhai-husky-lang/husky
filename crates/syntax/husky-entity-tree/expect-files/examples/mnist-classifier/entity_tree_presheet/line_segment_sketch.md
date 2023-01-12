Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `concave_component`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `convex_component`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `convexity`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convexity`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `line_segment`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `go_right`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `go_left`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `extend_end`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `extend_start`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `find_line_segments`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 163,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            1..2,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 164,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            4..5,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 165,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            6..7,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 166,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            8..9,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 167,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            10..11,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)