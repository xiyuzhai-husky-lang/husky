AstSheet {
    ast_arena: Arena {
        data: [
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                body: None,
            },
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
                        `syntax_basics::expr`,
                    ),
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        Ritchie(
                            Fn,
                        ),
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `nested`,
                    token_idx: TokenIdx(
                        2,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        3,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`syntax_basics::expr::nested`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        2..3,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            2..3,
        ),
    ],
}