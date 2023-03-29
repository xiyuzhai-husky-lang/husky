Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `core::clone`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            MethodFn,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `clone`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            1..2,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..1,
            ),
            ArenaIdxRange(
                1..2,
            ),
        ],
    },
)