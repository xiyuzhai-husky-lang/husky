AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Err {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                error: AstError::Original(
                    OriginalAstError::SubmoduleFileNotFound {
                        ident_token: IdentToken {
                            ident: `submodule_name`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        error: VfsError::FileNotExists(
                            VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../examples/basics/syntax-basics/src/ast/submodule_name.hsy",
                                ),
                            },
                        ),
                    },
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