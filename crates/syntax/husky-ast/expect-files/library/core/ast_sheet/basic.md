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
                                value: 3,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                state_after_visibility_expr: None,
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                ident: `derive`,
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
                                18,
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
                    ident: `bool`,
                    token_idx: TokenIdx(
                        20,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        21,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::bool`, `Extern`),
                    variants: None,
                },
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    3,
                ),
                ident: `derive`,
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
                                35,
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
                    ident: `never`,
                    token_idx: TokenIdx(
                        37,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        38,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::never`, `Extern`),
                    variants: None,
                },
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                ident: `derive`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    6,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                52,
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
                    ident: `unit`,
                    token_idx: TokenIdx(
                        54,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        55,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::unit`, `Extern`),
                    variants: None,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    7,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                56,
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
                    ident: `Trait`,
                    token_idx: TokenIdx(
                        58,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        59,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::Trait`, `Extern`),
                    variants: None,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    8,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                60,
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
                    ident: `Module`,
                    token_idx: TokenIdx(
                        62,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        63,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::Module`, `Extern`),
                    variants: None,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    9,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                64,
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
                    ident: `Lifetime`,
                    token_idx: TokenIdx(
                        66,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        67,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::Lifetime`, `Extern`),
                    variants: None,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    10,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                68,
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
                    ident: `Place`,
                    token_idx: TokenIdx(
                        70,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        71,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::basic::Place`, `Extern`),
                    variants: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..12,
    ),
    siblings: [
        ArenaIdxRange(
            1..12,
        ),
    ],
}