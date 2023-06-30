Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`core::logic::LogicOr`, `Inductive`),
                        ident: `Left`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            38,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Left`,
                        token_idx: TokenIdx(
                            39,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            40,
                        ),
                        drained: false,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`core::logic::LogicOr`, `Inductive`),
                        ident: `Right`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            43,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Right`,
                        token_idx: TokenIdx(
                            44,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            45,
                        ),
                        drained: false,
                    },
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
                            Extern,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Prop`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            3,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 8,
                            },
                        ),
                        variants: None,
                    },
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
                                    4,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `LogicAnd`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            7,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 9,
                            },
                        ),
                        variants: None,
                    },
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
                                    26,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `LogicOr`,
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            29,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 10,
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
            2..5,
        ),
        siblings: [
            ArenaIdxRange(
                2..5,
            ),
        ],
    },
)