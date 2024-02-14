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
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        6,
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
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
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
                    ident: `lambda_inline`,
                    token_idx: TokenIdx(
                        13,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        14,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`syntax_basics::expr::lambda_inline`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
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
                    ident: `lambda_nested`,
                    token_idx: TokenIdx(
                        29,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        30,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`syntax_basics::expr::lambda_nested`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                3..4,
                            ),
                        },
                    ),
                },
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: Some(
                        TokenIdx(
                            9,
                        ),
                    ),
                    raw: ShiftedU32(
                        1,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: Some(
                        TokenIdx(
                            41,
                        ),
                    ),
                    raw: ShiftedU32(
                        1,
                    ),
                },
                body: None,
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        4..7,
    ),
    nested_top_level_asts: [
        (
            TokenIdx(
                9,
            ),
            ArenaIdxRange(
                7..8,
            ),
        ),
        (
            TokenIdx(
                41,
            ),
            ArenaIdxRange(
                8..9,
            ),
        ),
    ],
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            2..2,
        ),
        ArenaIdxRange(
            2..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..4,
        ),
        ArenaIdxRange(
            4..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..8,
        ),
        ArenaIdxRange(
            8..8,
        ),
        ArenaIdxRange(
            8..9,
        ),
    ],
}