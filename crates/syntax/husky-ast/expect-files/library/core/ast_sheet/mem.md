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
                                TypePath(`core::mem::Ref`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
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
                                TypePath(`core::mem::RefMut`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
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
                                TypePath(`core::mem::Leash`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Leash`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                    is_generic: true,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        25,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
    },
)