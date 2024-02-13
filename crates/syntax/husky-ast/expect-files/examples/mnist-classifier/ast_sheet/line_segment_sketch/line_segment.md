AstSheet {
    ast_arena: Arena {
        data: [
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
                        10,
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
                            2..3,
                        ),
                    },
                ),
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
                        13,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            4..5,
                        ),
                    },
                ),
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
                        15,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            6..7,
                        ),
                    },
                ),
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        12,
                    ),
                },
                body: None,
            },
            Ast::IfElseStmts {
                if_branch: 5,
                elif_branches: ArenaIdxRange(
                    6..6,
                ),
                else_branch: Some(
                    7,
                ),
            },
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        11,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            8..10,
                        ),
                    },
                ),
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
                if_branch: 3,
                elif_branches: ArenaIdxRange(
                    4..4,
                ),
                else_branch: Some(
                    10,
                ),
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
                ident_token: IdentToken {
                    ident: `displacement`,
                    token_idx: TokenIdx(
                        24,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        25,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
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
                        6,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
                ident_token: IdentToken {
                    ident: `dist_to_point`,
                    token_idx: TokenIdx(
                        41,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        42,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                11..14,
                            ),
                        },
                    ),
                },
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
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Struct,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `LineSegment`,
                    token_idx: TokenIdx(
                        9,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        10,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    variants: None,
                },
            },
            Ast::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                items: Some(
                    Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                14..16,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        16..19,
    ),
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
            2..2,
        ),
        ArenaIdxRange(
            2..2,
        ),
        ArenaIdxRange(
            2..3,
        ),
        ArenaIdxRange(
            4..4,
        ),
        ArenaIdxRange(
            4..4,
        ),
        ArenaIdxRange(
            4..5,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..7,
        ),
        ArenaIdxRange(
            8..10,
        ),
        ArenaIdxRange(
            11..14,
        ),
        ArenaIdxRange(
            14..16,
        ),
        ArenaIdxRange(
            16..19,
        ),
    ],
}