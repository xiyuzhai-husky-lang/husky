Ok(
    AstSheet {
        arena: Arena {
            data: [
                Decor {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    error: UnexpectedStmtInsideModule,
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
                            Alien,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 15,
                                    },
                                ),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            22,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        23,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
    },
)