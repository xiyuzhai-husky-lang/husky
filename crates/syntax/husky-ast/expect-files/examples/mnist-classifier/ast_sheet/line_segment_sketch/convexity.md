AstSheet {
    ast_arena: Arena {
        data: [
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        13,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        14,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        18,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        19,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        10,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        11,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        12,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            1..3,
                        ),
                    },
                ),
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        15,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        16,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        17,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            3..5,
                        ),
                    },
                ),
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        20,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        9,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            5..12,
                        ),
                    },
                ),
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        22,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        21,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            13..14,
                        ),
                    },
                ),
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
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
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        7,
                    ),
                },
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        8,
                    ),
                },
                body: None,
            },
            Ast::IfElseStmts {
                if_branch: 12,
                elif_branches: ArenaIdxRange(
                    13..13,
                ),
                else_branch: Some(
                    14,
                ),
            },
            Ast::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convexity`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        FunctionFn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `is_convex`,
                    token_idx: TokenIdx(
                        21,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        22,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                15..20,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        20..24,
    ),
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..5,
        ),
        ArenaIdxRange(
            5..5,
        ),
        ArenaIdxRange(
            5..12,
        ),
        ArenaIdxRange(
            13..13,
        ),
        ArenaIdxRange(
            13..14,
        ),
        ArenaIdxRange(
            15..20,
        ),
        ArenaIdxRange(
            20..24,
        ),
    ],
}