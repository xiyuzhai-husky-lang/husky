Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::fermi`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `FermiMatchResult`,
                    accessibility: PubicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `fermi_match`,
                    accessibility: PubicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 21,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::fermi`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            0..1,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)