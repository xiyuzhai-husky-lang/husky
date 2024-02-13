AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `ilen`,
                    token_idx: TokenIdx(
                        31,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        32,
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
                        6,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                37,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `push`,
                    token_idx: TokenIdx(
                        39,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        40,
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
                        7,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                51,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `first`,
                    token_idx: TokenIdx(
                        53,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        54,
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
                        8,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                63,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `last`,
                    token_idx: TokenIdx(
                        65,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        66,
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
                        9,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                75,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `pop`,
                    token_idx: TokenIdx(
                        77,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        78,
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
                        10,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                87,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `collect_leashes`,
                    token_idx: TokenIdx(
                        89,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        90,
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
                        11,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                100,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `cyclic_slice_leashed`,
                    token_idx: TokenIdx(
                        102,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        103,
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
                        12,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                120,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: AssocItem {
                    assoc_item_kind: TypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `pop_with_largest_opt_f32`,
                    token_idx: TokenIdx(
                        122,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        123,
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
                        `core::vec`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                ident: `derive`,
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                14,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `Vec`,
                    token_idx: TokenIdx(
                        16,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        17,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::vec::Vec`, `Extern`),
                    variants: None,
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
                    Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                1..9,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        9..13,
    ),
    nested_top_level_asts: [],
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
            1..9,
        ),
        ArenaIdxRange(
            9..13,
        ),
    ],
}