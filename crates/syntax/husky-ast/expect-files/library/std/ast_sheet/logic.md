Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    0,
                                ),
                            },
                        },
                    },
                    state_after_visibility_expr: Some(
                        TokenStreamState {
                            next_token_idx: TokenIdx(
                                1,
                            ),
                            drained: false,
                        },
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..1,
        ),
        siblings: [
            ArenaIdxRange(
                0..1,
            ),
        ],
    },
)