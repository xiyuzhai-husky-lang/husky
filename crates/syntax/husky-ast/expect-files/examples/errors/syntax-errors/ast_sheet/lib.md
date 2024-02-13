AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_errors`,
                    ),
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `ast`,
                    token_idx: TokenIdx(
                        2,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        3,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_errors::ast`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_errors`,
                    ),
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `uses`,
                    token_idx: TokenIdx(
                        4,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        5,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_errors::uses`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..3,
    ),
    siblings: [
        ArenaIdxRange(
            1..3,
        ),
    ],
}