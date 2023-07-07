Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        ident: `ConfidentYes`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            11,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ConfidentYes`,
                        token_idx: TokenIdx(
                            12,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            13,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        ident: `ConfidentNo`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            13,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ConfidentNo`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            15,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        ident: `Unconfident`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            15,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Unconfident`,
                        token_idx: TokenIdx(
                            16,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            17,
                        ),
                        drained: true,
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
                            Enum,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `OneVsAllResult`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            3,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 57,
                            },
                        ),
                        variants: Some(
                            TypeVariants {
                                ast_idx_range: ArenaIdxRange(
                                    0..3,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
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
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Gn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `narrow_down`,
                        token_idx: TokenIdx(
                            19,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            20,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 78,
                            },
                        ),
                        body: None,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            3..5,
        ),
        siblings: [
            ArenaIdxRange(
                3..3,
            ),
            ArenaIdxRange(
                3..5,
            ),
        ],
    },
)