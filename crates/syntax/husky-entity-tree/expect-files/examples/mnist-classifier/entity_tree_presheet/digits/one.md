Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::one`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `one_fermi_match`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::one`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 68,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_one`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                            accessibility: Public,
                            ast_idx: 69,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `upmost`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::one`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 70,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `downmost`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::one`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 71,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `hat`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::one`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            ast_idx: 72,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 61,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 121,
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
                    ast_idx: 62,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 63,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            12..13,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 64,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 105,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            17..18,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 65,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    39,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            20..21,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 66,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            23..24,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 67,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::one`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            27..28,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)