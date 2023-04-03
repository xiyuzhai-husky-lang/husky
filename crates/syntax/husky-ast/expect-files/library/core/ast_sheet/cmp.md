Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedTraitItems(
                            TokenGroupIdx(
                                1,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedTraitItems(
                            TokenGroupIdx(
                                2,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedTraitItems(
                            TokenGroupIdx(
                                3,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedTraitItems(
                            TokenGroupIdx(
                                4,
                            ),
                        ),
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..4,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..4,
            ),
        ],
    },
)