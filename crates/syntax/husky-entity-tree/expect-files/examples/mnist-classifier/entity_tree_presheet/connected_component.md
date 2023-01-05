Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::connected_component`,
        module_symbols: [
            ModuleItem {
                ident: `ConnectedComponentDistribution`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 116,
                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            },
            ModuleItem {
                ident: `EffHoles`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 117,
                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            },
            ModuleItem {
                ident: `hole_tmpl`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 118,
                path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
            },
            ModuleItem {
                ident: `ConnectedComponent`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 119,
                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            },
            ModuleItem {
                ident: `horizontal_extend`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 120,
                path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
            },
            ModuleItem {
                ident: `find_connected_components`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 121,
                path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 114,
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
                    ast_idx: 115,
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