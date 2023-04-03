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
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..1,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..1,
            ),
        ],
    },
)