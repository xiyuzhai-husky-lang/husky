Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
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
                    item_kind: MajorItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `PartialEq`,
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
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 2,
                            },
                        ),
                        items: None,
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
                    item_kind: MajorItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Eq`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            7,
                        ),
                        drained: false,
                    },
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 3,
                            },
                        ),
                        items: None,
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
                                    8,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `PartialOrd`,
                        token_idx: TokenIdx(
                            10,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            11,
                        ),
                        drained: false,
                    },
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 4,
                            },
                        ),
                        items: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    12,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Ord`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            15,
                        ),
                        drained: false,
                    },
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 5,
                            },
                        ),
                        items: None,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..4,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..4,
            ),
        ],
    },
)