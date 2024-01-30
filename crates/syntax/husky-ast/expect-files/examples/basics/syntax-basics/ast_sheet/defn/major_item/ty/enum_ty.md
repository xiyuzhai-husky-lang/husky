AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Err {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                error: AstError::Original(
                    OriginalAstError::ExpectedTypeVariants(
                        TokenGroupIdx(
                            1,
                        ),
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..2,
    ),
    siblings: [
        ArenaIdxRange(
            1..2,
        ),
    ],
}