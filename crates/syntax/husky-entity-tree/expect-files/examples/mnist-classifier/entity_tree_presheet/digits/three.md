Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits::three`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `three_fermi_match`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 34,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `is_three`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 35,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `uparc`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 36,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `downarc`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 37,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `back`,
                    accessibility: PubicUnder(
                        `mnist_classifier::digits::three`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::digits::three::back`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            ast_idx: 38,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 27,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 103,
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
                    ast_idx: 28,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 103,
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
                    ast_idx: 29,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 103,
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
                    ast_idx: 30,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 103,
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
                    ast_idx: 31,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
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
                    ast_idx: 32,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
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
                    ast_idx: 33,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::digits::three`,
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
            ],
        ),
    },
)