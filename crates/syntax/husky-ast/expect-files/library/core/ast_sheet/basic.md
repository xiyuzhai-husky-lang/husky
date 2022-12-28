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
                            Alias,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath {
                            module: `core::basic`,
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 7,
                                    },
                                ),
                            ),
                        },
                    ),
                    ident: `bool`,
                    is_generic: false,
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
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath {
                            module: `core::basic`,
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 8,
                                    },
                                ),
                            ),
                        },
                    ),
                    ident: `Trait`,
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
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
                        TypePath {
                            module: `core::basic`,
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 9,
                                    },
                                ),
                            ),
                        },
                    ),
                    ident: `Module`,
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)