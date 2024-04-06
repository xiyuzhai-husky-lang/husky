```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 5,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 8,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 9,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 10,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 11,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 12,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 13,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 14,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 7,
                },
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            3..10,
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
                    raw: 15,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 16,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 17,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 18,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 19,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 20,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 21,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 22,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 23,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 24,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 25,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 26,
                },
                body: None,
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::zero`,
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Fugitive(
                        MajorFugitiveKind::Val,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `open_one_match`,
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
                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                    raw: 3,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::zero`,
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
                    ident: `almost_closed`,
                    token_idx: TokenIdx(
                        19,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        20,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..3,
                            ),
                        },
                    ),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 6,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::PubUnder {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                45,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                46,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    47,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                48,
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
                    ident: `is_zero`,
                    token_idx: TokenIdx(
                        50,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        51,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                11..24,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        24..28,
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
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..10,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..24,
        ),
        ArenaIdxRange(
            24..28,
        ),
    ],
}
```