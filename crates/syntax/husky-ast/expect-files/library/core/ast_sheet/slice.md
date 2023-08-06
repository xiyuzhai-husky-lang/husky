Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        },
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `ilen`,
                        token_idx: TokenIdx(
                            29,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            30,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        5,
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
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `start`,
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
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    43,
                                ),
                            },
                        },
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `end`,
                        token_idx: TokenIdx(
                            45,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            46,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        },
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `first`,
                        token_idx: TokenIdx(
                            53,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            54,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        },
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `last`,
                        token_idx: TokenIdx(
                            63,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            64,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
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
                                    value: 18,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    state_after_visibility_expr: None,
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
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Slice`,
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
                                value: 32,
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
                                    12,
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
                        ident: `CyclicSliceLeashed`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            15,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 33,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    items: Some(
                        Type(
                            TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    0..5,
                                ),
                            },
                        ),
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            5..9,
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
                0..0,
            ),
            ArenaIdxRange(
                0..5,
            ),
            ArenaIdxRange(
                5..9,
            ),
        ],
    },
)