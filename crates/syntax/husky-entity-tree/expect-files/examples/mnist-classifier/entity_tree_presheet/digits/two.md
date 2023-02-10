Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::two`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `two_match`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 62,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `left_cc_pattern`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 63,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `right_cc_pattern`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 64,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `down_cc_pattern`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 65,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_two`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::two`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            ast_idx: 66,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 49,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
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
                            4..5,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 50,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
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
                                    13,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 51,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
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
                                    23,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            14..15,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 52,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
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
                                    33,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            19..20,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 53,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    43,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            22..23,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 54,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            25..26,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 55,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            29..30,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 56,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    63,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            32..33,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 57,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    69,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            35..36,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 58,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    75,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            38..39,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 59,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    81,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            41..42,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 60,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    87,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            44..45,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 61,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::two`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    93,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            47..48,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)