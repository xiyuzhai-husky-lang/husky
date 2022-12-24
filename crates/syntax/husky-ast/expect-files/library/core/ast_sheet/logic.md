Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem(
                        Type(
                            Structure,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::logic::LogicAnd`,
                        ),
                    ),
                    ident: `LogicAnd`,
                    is_generic: true,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem(
                        Type(
                            Inductive,
                        ),
                    ),
                    entity_path: Some(
                        ModuleItem(
                            `core::logic::LogicOr`,
                        ),
                    ),
                    ident: `LogicOr`,
                    is_generic: true,
                    body_kind: EnumVariants,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..4,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)