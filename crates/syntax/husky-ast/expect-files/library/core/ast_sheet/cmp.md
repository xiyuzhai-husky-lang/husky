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
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Trait(
                                TraitPath(`core::cmp::PartialEq`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `PartialEq`,
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
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Trait(
                                TraitPath(`core::cmp::Eq`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Eq`,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Trait(
                                TraitPath(`core::cmp::PartialOrd`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `PartialOrd`,
                        token_idx: TokenIdx(
                            10,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        11,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Ord`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        15,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..4,
        ),
    },
)