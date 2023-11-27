AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Use {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                state_after_visibility_expr: None,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
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
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
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
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    3,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                13,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
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
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    4,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                17,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Trait,
                    connection: Connected,
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
        1..6,
    ),
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
            1..6,
        ),
    ],
}