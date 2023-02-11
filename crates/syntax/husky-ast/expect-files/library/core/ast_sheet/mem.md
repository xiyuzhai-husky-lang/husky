Ok(
    AstSheet {
        arena: Arena {
            data: [
                Ast::Defn {
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
                        TypePath(`core::mem::Ref`, `Alien`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Ref`,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
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
                        TypePath(`core::mem::RefMut`, `Alien`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `RefMut`,
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    is_generic: true,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        14,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..2,
        ),
    },
)