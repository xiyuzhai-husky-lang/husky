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
                    use_expr_children: Some(
                        ArenaIdxRange(
                            1..2,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 1,
                    accessibility: Done {
                        accessibility: Public,
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            4..5,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)