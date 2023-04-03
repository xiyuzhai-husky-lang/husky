Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `clone`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                    block: AssociatedItem {
                        body: None,
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
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                    block: Trait {
                        path: TraitPath(
                            Id {
                                value: 1,
                            },
                        ),
                        items: TraitItems {
                            children: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            1..2,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..1,
            ),
            ArenaIdxRange(
                1..2,
            ),
        ],
    },
)