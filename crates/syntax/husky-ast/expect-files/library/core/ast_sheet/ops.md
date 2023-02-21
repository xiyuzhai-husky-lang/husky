Ok(
    AstSheet {
        arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PubicUnder(
                        `core::ops`,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PubicUnder(
                        `core::ops`,
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
                            18,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        19,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PubicUnder(
                        `core::ops`,
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
                            44,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        45,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PubicUnder(
                        `core::ops`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitItem(
                            Method,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `sub`,
                        token_idx: TokenIdx(
                            47,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        48,
                    ),
                },
                Ast::Decor {
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
                Ast::Decor {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        2..4,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        TraitPath(`core::ops::Sub`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Sub`,
                        token_idx: TokenIdx(
                            38,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        39,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            4..8,
        ),
    },
)