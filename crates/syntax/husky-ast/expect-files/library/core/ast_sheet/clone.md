```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `core::clone`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitItem(
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `clone`,
                    token_idx: TokenIdx(
                        10,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        11,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
                    body: None,
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
                        `core::clone`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitForTypeItem(
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `clone`,
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
                    body: None,
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
                        `core::clone`,
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
                    ident: `Clone`,
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
                    path: TraitPath(`core::clone::Clone`),
                    items: Some(
                        TraitItems {
                            ast_idx_range: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
                },
            },
            AstData::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
                },
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        2..5,
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
            1..2,
        ),
        ArenaIdxRange(
            2..5,
        ),
    ],
}
```