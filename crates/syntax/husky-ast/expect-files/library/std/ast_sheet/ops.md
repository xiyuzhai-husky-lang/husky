Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std::ops`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            AssociatedType,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `Output`,
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        14,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std::ops`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            Method,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `add`,
                        token_idx: TokenIdx(
                            16,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        17,
                    ),
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `std::ops`,
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Trait(
                                TraitPath(`std::ops::Add`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        8,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..4,
        ),
    },
)