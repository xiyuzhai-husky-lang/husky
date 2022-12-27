Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::connected_component`,
        module_symbols: [
            ModuleItem {
                ident: `ConnectedComponentDistribution`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 121,
                path: `mnist_classifier::connected_component::ConnectedComponentDistribution`,
            },
            ModuleItem {
                ident: `EffHoles`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 122,
                path: `mnist_classifier::connected_component::EffHoles`,
            },
            ModuleItem {
                ident: `hole_tmpl`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 123,
                path: `mnist_classifier::connected_component::hole_tmpl`,
            },
            ModuleItem {
                ident: `ConnectedComponent`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 124,
                path: `mnist_classifier::connected_component::ConnectedComponent`,
            },
            ModuleItem {
                ident: `horizontal_extend`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 125,
                path: `mnist_classifier::connected_component::horizontal_extend`,
            },
            ModuleItem {
                ident: `find_connected_components`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 126,
                path: `mnist_classifier::connected_component::find_connected_components`,
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 119,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 102,
                            },
                        ),
                    ),
                    use_expr_idx: 3,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 120,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
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
            ],
        ),
    },
)