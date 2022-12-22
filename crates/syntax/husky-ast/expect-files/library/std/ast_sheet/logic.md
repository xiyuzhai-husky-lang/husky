Ok(
    AstSheet {
        arena: Arena {
            data: [
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `core`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..1,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `logic`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `core`,
                    child: 1,
                },
            ],
        },
    },
)