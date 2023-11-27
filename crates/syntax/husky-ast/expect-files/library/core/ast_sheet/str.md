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
                                value: 18,
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
                    ident: `str`,
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
                block: DefnBlock::Type {
                    path: TypePath(`core::str::str`, `Extern`),
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
                                9,
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
                    ident: `StringLiteral`,
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
                block: DefnBlock::Type {
                    path: TypePath(`core::str::StringLiteral`, `Extern`),
                    variants: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..4,
    ),
    siblings: [
        ArenaIdxRange(
            1..4,
        ),
    ],
}