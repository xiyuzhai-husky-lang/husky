Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::connected_component`,
        module_symbols: [
            ModuleItem {
                ident: `ConnectedComponentDistribution`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 10,
                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            },
            ModuleItem {
                ident: `EffHoles`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 11,
                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            },
            ModuleItem {
                ident: `hole_tmpl`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 12,
                path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
            },
            ModuleItem {
                ident: `ConnectedComponent`,
                accessibility: PubicUnder(
                    `mnist_classifier::connected_component`,
                ),
                ast_idx: 14,
                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 8,
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
                    ast_idx: 9,
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