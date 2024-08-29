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
                    raw: 3,
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
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`syntax_basics::expr`),
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
                block: DefnBlock::Form {
                    path: MajorFormPath(`syntax_basics::expr::nested`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
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
                    raw: 2,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`syntax_basics::expr`),
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
                    ident: `closure_inline`,
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
                block: DefnBlock::Form {
                    path: MajorFormPath(`syntax_basics::expr::closure_inline`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`syntax_basics::expr`),
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
                    ident: `closure_nested`,
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
                block: DefnBlock::Form {
                    path: MajorFormPath(`syntax_basics::expr::closure_nested`, `Ritchie(
                        Fn,
                    )`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                },
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: Some(
                        TokenIdx(
                            9,
                        ),
                    ),
                    raw: 0,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: Some(
                        TokenIdx(
                            41,
                        ),
                    ),
                    raw: 0,
                },
                body: None,
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        3..6,
    ),
    nested_top_level_asts: [
        (
            TokenIdx(
                9,
            ),
            ArenaIdxRange(
                6..7,
            ),
        ),
        (
            TokenIdx(
                41,
            ),
            ArenaIdxRange(
                7..8,
            ),
        ),
    ],
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
            1..2,
        ),
        ArenaIdxRange(
            2..2,
        ),
        ArenaIdxRange(
            2..3,
        ),
        ArenaIdxRange(
            3..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..8,
        ),
    ],
}
```