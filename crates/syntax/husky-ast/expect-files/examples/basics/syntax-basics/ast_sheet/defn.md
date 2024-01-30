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
                        `syntax_basics::defn`,
                    ),
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `major_item`,
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
                                        `syntax_basics::defn::major_item`,
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
        1..2,
    ),
    siblings: [
        ArenaIdxRange(
            1..2,
        ),
    ],
}