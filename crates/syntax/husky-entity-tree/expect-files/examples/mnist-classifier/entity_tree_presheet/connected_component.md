Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::connected_component`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `EffHoles`,
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `hole_tmpl`,
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `horizontal_extend`,
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `find_connected_components`,
                    accessibility: PubicUnder(
                        `mnist_classifier::connected_component`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 118,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            2..3,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 119,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::connected_component`,
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
            ],
        ),
    },
)