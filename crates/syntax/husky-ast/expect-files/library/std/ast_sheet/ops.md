Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 23,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            AssociatedType,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `Output`,
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            14,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 23,
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
                        ident: `add`,
                        token_idx: TokenIdx(
                            16,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            17,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Hint {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 23,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Add`,
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
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 27,
                            },
                        ),
                        items: Some(
                            TraitItems {
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
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..2,
            ),
            ArenaIdxRange(
                2..4,
            ),
        ],
    },
)