Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            accessibility: Public,
                            ast_idx: 73,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `find_concave_components`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch::concave_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            ast_idx: 75,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 69,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
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
                    ast_idx: 70,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            5..6,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 71,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            9..10,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 72,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            12..13,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)