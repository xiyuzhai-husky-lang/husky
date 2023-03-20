Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::str::str`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `str`,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::str::StringLiteral`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `StringLiteral`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        7,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..2,
        ),
    },
)