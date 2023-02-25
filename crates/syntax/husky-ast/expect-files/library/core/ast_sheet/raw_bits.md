Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `r32`,
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