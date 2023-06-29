Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    variant_path: TypeVariantPath {
                        ty_path: TypePath(`core::option::Option`, `Enum`),
                        ident: `Some`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            11,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Some`,
                        token_idx: TokenIdx(
                            12,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            13,
                        ),
                        drained: false,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    variant_path: TypeVariantPath {
                        ty_path: TypePath(`core::option::Option`, `Enum`),
                        ident: `None`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            16,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `None`,
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            18,
                        ),
                        drained: true,
                    },
                },
                Ast::Use {
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
                    state_after_visibility_expr: Some(
                        TokenStreamState {
                            next_token_idx: TokenIdx(
                                1,
                            ),
                            drained: false,
                        },
                    ),
                },
                Ast::Defn {
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
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Enum,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Option`,
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
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 29,
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
            2..4,
        ),
        siblings: [
            ArenaIdxRange(
                2..4,
            ),
        ],
    },
)