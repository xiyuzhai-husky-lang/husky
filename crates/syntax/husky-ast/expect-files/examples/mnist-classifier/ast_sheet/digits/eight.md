```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        8,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        7,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            1..2,
                        ),
                    },
                ),
            },
            AstData::IfElseStmts {
                if_branch: 2,
                elif_branches: ArenaIdxRange(
                    3..3,
                ),
                else_branch: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        9,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        6,
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
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
                },
                body: None,
            },
            AstData::IfElseStmts {
                if_branch: 5,
                elif_branches: ArenaIdxRange(
                    6..6,
                ),
                else_branch: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        10,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        13,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        12,
                    ),
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            9..10,
                        ),
                    },
                ),
            },
            AstData::IfElseStmts {
                if_branch: 10,
                elif_branches: ArenaIdxRange(
                    11..11,
                ),
                else_branch: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        14,
                    ),
                },
                body: None,
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Fugitive(
                        MajorFugitiveKind::Val,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `upper_mouth_match`,
                    token_idx: TokenIdx(
                        6,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        7,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::PubUnder {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                19,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                21,
                            ),
                        ),
                    },
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Fugitive(
                        MajorFugitiveKind::Val,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `is_eight`,
                    token_idx: TokenIdx(
                        23,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        24,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                6..9,
                            ),
                        },
                    ),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        11,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::eight`,
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Fugitive(
                        MajorFugitiveKind::Ritchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `big_mouth`,
                    token_idx: TokenIdx(
                        73,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        74,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                11..13,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        13..17,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..0,
        ),
        ArenaIdxRange(
            0..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..5,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..9,
        ),
        ArenaIdxRange(
            9..9,
        ),
        ArenaIdxRange(
            9..10,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..13,
        ),
        ArenaIdxRange(
            13..17,
        ),
    ],
}
```