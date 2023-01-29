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
                            Alien,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
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
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 22,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: true,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        7,
                    ),
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
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 27,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                    is_generic: true,
                    body_kind: EnumVariants,
                    saved_stream_state: TokenIdx(
                        29,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    error: UnexpectedStmtInsideModule,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: UnexpectedStmtInsideModule,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..5,
        ),
    },
)