AstSheet {
    ast_arena: Arena {
        data: [
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    4,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 18,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        43,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Left`,
                    token_idx: TokenIdx(
                        44,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        45,
                    ),
                    drained: false,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 19,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        48,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Right`,
                    token_idx: TokenIdx(
                        49,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        50,
                    ),
                    drained: false,
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
                                value: 8,
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
                    ident: `Prop`,
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
                    path: TypePath(`core::logic::Prop`, `Extern`),
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
                        Structure,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `LogicAnd`,
                    token_idx: TokenIdx(
                        11,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        12,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::logic::LogicAnd`, `Structure`),
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
                                31,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Inductive,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `LogicOr`,
                    token_idx: TokenIdx(
                        33,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        34,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::logic::LogicOr`, `Inductive`),
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