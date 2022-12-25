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
                        item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `core::logic`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                    ),
                                },
                            ),
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
                    entity_kind: ModuleItem {
                        item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `core::logic`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 16,
                                            },
                                        ),
                                    ),
                                },
                            ),
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