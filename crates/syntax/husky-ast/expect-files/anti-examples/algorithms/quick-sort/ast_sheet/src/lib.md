```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                body: None,
            },
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
                    raw: 5,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 6,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 7,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            2..5,
                        ),
                    },
                ),
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
                    raw: 15,
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
                    raw: 20,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 19,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            9..10,
                        ),
                    },
                ),
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
                    raw: 21,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            11..12,
                        ),
                    },
                ),
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
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            7..8,
                        ),
                    },
                ),
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
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            8..9,
                        ),
                    },
                ),
            },
            AstData::IfElseStmts {
                if_branch: 10,
                elif_branches: ArenaIdxRange(
                    11..11,
                ),
                else_branch: Some(
                    12,
                ),
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
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            13..18,
                        ),
                    },
                ),
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
                    raw: 27,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 30,
                },
                body: None,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                1,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Ritchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort`,
                    token_idx: TokenIdx(
                        3,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        4,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: MajorFormPath(`quick_sort::quick_sort`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                0..2,
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
                        ModulePath(`quick_sort`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Ritchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort_aux`,
                    token_idx: TokenIdx(
                        43,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        44,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                6..7,
                            ),
                        },
                    ),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 8,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`quick_sort`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Ritchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `partition`,
                    token_idx: TokenIdx(
                        105,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        106,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: MajorFormPath(`quick_sort::partition`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                18..24,
                            ),
                        },
                    ),
                },
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 25,
                },
                ident: `test`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 26,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`quick_sort`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Ritchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort_works_for_integers`,
                    token_idx: TokenIdx(
                        231,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        232,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: MajorFormPath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                24..25,
                            ),
                        },
                    ),
                },
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 28,
                },
                ident: `test`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 29,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`quick_sort`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Ritchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort_works_for_strs`,
                    token_idx: TokenIdx(
                        268,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        269,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: MajorFormPath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                25..26,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        26..33,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..0,
        ),
        ArenaIdxRange(
            0..0,
        ),
        ArenaIdxRange(
            0..2,
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
            2..5,
        ),
        ArenaIdxRange(
            6..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..7,
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
            8..8,
        ),
        ArenaIdxRange(
            8..9,
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
            11..12,
        ),
        ArenaIdxRange(
            13..18,
        ),
        ArenaIdxRange(
            18..18,
        ),
        ArenaIdxRange(
            18..18,
        ),
        ArenaIdxRange(
            18..24,
        ),
        ArenaIdxRange(
            24..24,
        ),
        ArenaIdxRange(
            24..25,
        ),
        ArenaIdxRange(
            25..25,
        ),
        ArenaIdxRange(
            25..26,
        ),
        ArenaIdxRange(
            26..33,
        ),
    ],
}
```