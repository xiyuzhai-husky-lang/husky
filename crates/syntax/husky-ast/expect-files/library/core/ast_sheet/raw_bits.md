Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Attr {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    ident: `derive`,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `r32`,
                        token_idx: TokenIdx(
                            22,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        23,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 28,
                            },
                        ),
                        variants: None,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
        siblings: [
            ArenaIdxRange(
                0..3,
            ),
        ],
    },
)