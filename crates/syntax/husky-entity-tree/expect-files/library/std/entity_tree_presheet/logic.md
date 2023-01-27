Ok(
    EntityTreePresheet {
        module_path: `std::logic`,
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
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 12,
                                        },
                                    ),
                                ),
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
            ],
        ),
    },
)