AstSheet {
    ast_arena: Arena {
        data: [
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    4,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 74,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 115,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        29,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Some`,
                    token_idx: TokenIdx(
                        30,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        31,
                    ),
                    drained: false,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 74,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 116,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        34,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `None`,
                    token_idx: TokenIdx(
                        35,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        36,
                    ),
                    drained: true,
                },
            },
            Ast::Use {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                state_after_visibility_expr: None,
            },
            Ast::Use {
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
                state_after_visibility_expr: Some(
                    TokenStreamState {
                        next_token_idx: TokenIdx(
                            6,
                        ),
                        drained: false,
                    },
                ),
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                ident: `derive`,
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
                                23,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `Option`,
                    token_idx: TokenIdx(
                        25,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        26,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::option::Option`, `Enum`),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                1..3,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        3..7,
    ),
    siblings: [
        ArenaIdxRange(
            3..7,
        ),
    ],
}