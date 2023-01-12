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
                UseTracker {
                    ast_idx: 21,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        0..1,
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)