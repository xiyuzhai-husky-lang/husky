Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            AssociatedType,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `Output`,
                        token_idx: TokenIdx(
                            15,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        16,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            Method,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `add`,
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        18,
                    ),
                },
                Decor {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        TraitPath(`core::ops::Add`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            9,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        10,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..4,
        ),
    },
)