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
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 120,
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
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 121,
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
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 122,
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
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 123,
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
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 125,
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
                            accessibility: PubicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            ast_idx: 126,
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
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 107,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            2..3,
                        ),
                    },
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
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            5..6,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)