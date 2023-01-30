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
                        TypePath(`core::vec::Vec`, `Alien`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Vec`,
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
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..1,
        ),
    },
)