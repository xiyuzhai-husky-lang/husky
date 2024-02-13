AstSheet {
    ast_arena: Arena {
        data: [
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
                        `core::clone`,
                    ),
                },
                item_kind: AssocItem {
                    assoc_item_kind: TraitItem(
                        MethodFn,
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
                        `core::clone`,
                    ),
                },
                item_kind: AssocItem {
                    assoc_item_kind: TraitForTypeItem(
                        MethodFn,
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
                        `core::clone`,
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
                                5,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
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
                                1..2,
                            ),
                        },
                    ),
                },
            },
            Ast::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        3..6,
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
            2..3,
        ),
        ArenaIdxRange(
            3..6,
        ),
    ],
}