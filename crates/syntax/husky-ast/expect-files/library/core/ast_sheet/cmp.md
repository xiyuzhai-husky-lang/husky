```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `core::cmp`,
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
                                5,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Trait,
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `PartialEq`,
                    token_idx: TokenIdx(
                        7,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        8,
                    ),
                    drained: false,
                },
                block: DefnBlock::Trait {
                    path: TraitPath(`core::cmp::PartialEq`),
                    items: None,
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Trait,
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `Eq`,
                    token_idx: TokenIdx(
                        11,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        12,
                    ),
                    drained: false,
                },
                block: DefnBlock::Trait {
                    path: TraitPath(`core::cmp::Eq`),
                    items: None,
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                13,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Trait,
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `PartialOrd`,
                    token_idx: TokenIdx(
                        15,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        16,
                    ),
                    drained: false,
                },
                block: DefnBlock::Trait {
                    path: TraitPath(`core::cmp::PartialOrd`),
                    items: None,
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                17,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Trait,
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `Ord`,
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
                block: DefnBlock::Trait {
                    path: TraitPath(`core::cmp::Ord`),
                    items: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        0..5,
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
            0..0,
        ),
        ArenaIdxRange(
            0..0,
        ),
        ArenaIdxRange(
            0..5,
        ),
    ],
}
```