Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `std::prelude`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `prelude`,
                        token_idx: TokenIdx(
                            1,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        2,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
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
                                    value: 17,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `std::logic`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `logic`,
                        token_idx: TokenIdx(
                            3,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        4,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `std::ops`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
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
                0..3,
            ),
        ],
    },
)