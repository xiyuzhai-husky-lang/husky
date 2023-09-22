Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Identifiable {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 20,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `visualize`,
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
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Identifiable {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 20,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `visualize`,
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            29,
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
                                    value: 20,
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
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Visualize`,
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
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 26,
                            },
                        ),
                        items: Some(
                            TraitItems {
                                ast_idx_range: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                        ),
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
                                    16,
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
                        ident: `Html`,
                        token_idx: TokenIdx(
                            18,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            19,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 39,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                        ),
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            3..7,
        ),
        siblings: [
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..3,
            ),
            ArenaIdxRange(
                3..7,
            ),
        ],
    },
)