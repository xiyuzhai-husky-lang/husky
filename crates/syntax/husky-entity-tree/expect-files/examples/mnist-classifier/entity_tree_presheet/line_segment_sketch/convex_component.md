Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `ConvexComponent`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 2,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convex_component`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)