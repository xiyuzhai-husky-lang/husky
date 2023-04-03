Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    path: TypeVariantPath {
                        path: TypePath(`core::option::Option`, `Enum`),
                        ident: `Some`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            6,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Some`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    state_after: TokenIdx(
                        8,
                    ),
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    path: TypeVariantPath {
                        path: TypePath(`core::option::Option`, `Enum`),
                        ident: `None`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            11,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `None`,
                        token_idx: TokenIdx(
                            12,
                        ),
                    },
                    state_after: TokenIdx(
                        13,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    0,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Enum,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Option`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 27,
                            },
                        ),
                        variants: Some(
                            TypeVariants {
                                ast_idx_range: ArenaIdxRange(
                                    0..2,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..3,
        ),
        siblings: [
            ArenaIdxRange(
                2..3,
            ),
        ],
    },
)