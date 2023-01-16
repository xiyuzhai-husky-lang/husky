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
                            accessibility: PubicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 22,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `fermi_match`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                            accessibility: Public,
                            ast_idx: 24,
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
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)