Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 7,
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
                        ident: `Debug`,
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
                                value: 6,
                            },
                        ),
                        items: None,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            1..3,
        ),
        siblings: [
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..3,
            ),
        ],
    },
)