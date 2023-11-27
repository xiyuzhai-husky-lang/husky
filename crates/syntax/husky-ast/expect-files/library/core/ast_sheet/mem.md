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
                                value: 10,
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
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `Ref`,
                    token_idx: TokenIdx(
                        7,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        8,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::mem::Ref`, `Extern`),
                    variants: None,
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
                                16,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `RefMut`,
                    token_idx: TokenIdx(
                        18,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        19,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::mem::RefMut`, `Extern`),
                    variants: None,
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
                                27,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `Leash`,
                    token_idx: TokenIdx(
                        29,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        30,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::mem::Leash`, `Extern`),
                    variants: None,
                },
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    4,
                ),
                items: None,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                44,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `At`,
                    token_idx: TokenIdx(
                        46,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        47,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::mem::At`, `Extern`),
                    variants: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..7,
    ),
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..7,
        ),
    ],
}