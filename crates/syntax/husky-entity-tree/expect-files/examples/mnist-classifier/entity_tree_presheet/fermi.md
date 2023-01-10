Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::fermi`,
        module_specific_symbols: [
            ModuleItem {
                ident: `FermiMatchResult`,
                accessibility: PubicUnder(
                    `mnist_classifier::fermi`,
                ),
                ast_idx: 22,
                path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
            },
            ModuleItem {
                ident: `fermi_match`,
                accessibility: PubicUnder(
                    `mnist_classifier::fermi`,
                ),
                ast_idx: 24,
                path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 21,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    use_expr_idx: 1,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)