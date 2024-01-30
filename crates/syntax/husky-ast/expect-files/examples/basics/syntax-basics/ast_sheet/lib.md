AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_basics`,
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
                                        `syntax_basics::ast`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_basics`,
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
                                        `syntax_basics::uses`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_basics`,
                    ),
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `defn`,
                    token_idx: TokenIdx(
                        6,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        7,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_basics::defn`,
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
        1..4,
    ),
    siblings: [
        ArenaIdxRange(
            1..4,
        ),
    ],
}