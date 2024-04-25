```rust
AstSheet {
    ast_arena: Arena {
        data: [
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
                    raw: 9,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 8,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            1..2,
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
                    raw: 12,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            3..4,
                        ),
                    },
                ),
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
                    raw: 14,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            5..6,
                        ),
                    },
                ),
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 11,
                },
                body: None,
            },
            AstData::IfElseStmts {
                if_branch: 4,
                elif_branches: ArenaIdxRange(
                    5..5,
                ),
                else_branch: Some(
                    6,
                ),
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 10,
                },
                body: Some(
                    FormBody {
                        ast_idx_range: ArenaIdxRange(
                            7..9,
                        ),
                    },
                ),
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
            AstData::IfElseStmts {
                if_branch: 2,
                elif_branches: ArenaIdxRange(
                    3..3,
                ),
                else_branch: Some(
                    9,
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TypeItem(
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
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
                    raw: 5,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TypeItem(
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
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
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                10..13,
                            ),
                        },
                    ),
                },
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::line_segment`,
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
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Struct,
                    ),
                    connection: MajorItemConnectionKind::Connected,
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
            AstData::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                items: Some(
                    Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                13..15,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        15..18,
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
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..4,
        ),
        ArenaIdxRange(
            5..5,
        ),
        ArenaIdxRange(
            5..6,
        ),
        ArenaIdxRange(
            7..9,
        ),
        ArenaIdxRange(
            10..13,
        ),
        ArenaIdxRange(
            13..15,
        ),
        ArenaIdxRange(
            15..18,
        ),
    ],
}
```