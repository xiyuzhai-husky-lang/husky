Ok(
    EntityTreePresheet {
        module_path: `core::prelude`,
        module_specific_symbols: NativeEntitySymbolTable(
            [],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 0,
                    accessibility: Done {
                        accessibility: Public,
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 1,
                    accessibility: Done {
                        accessibility: Public,
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    9,
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
                    ast_idx: 2,
                    accessibility: Done {
                        accessibility: Public,
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    16,
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
            ],
        ),
    },
)