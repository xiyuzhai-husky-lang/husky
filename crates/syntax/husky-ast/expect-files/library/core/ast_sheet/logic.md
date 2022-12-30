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
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`core::logic::LogicAnd, Structure`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `LogicAnd`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`core::logic::LogicOr, Inductive`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `LogicOr`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                    is_generic: true,
                    body_kind: EnumVariants,
                    saved_stream_state: TokenIdx(
                        25,
                    ),
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