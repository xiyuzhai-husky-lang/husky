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
                                TypePath(`core::logic::Prop`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Prop`,
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
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::logic::LogicAnd`, `Structure`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `LogicAnd`,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::logic::LogicOr`, `Inductive`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `LogicOr`,
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
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..5,
        ),
    },
)