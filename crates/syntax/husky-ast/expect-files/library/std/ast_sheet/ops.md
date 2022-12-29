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
                        item_kind: Type(
                            Foreign,
                        ),
                    },
                    entity_path: Some(
                        AssociatedItemPath {
                            parent_path: TraitPath(`std::ops::Add`),
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 30,
                                    },
                                ),
                            ),
                        },
                    ),
                    ident_token: IdentifierToken {
                        ident: `Output`,
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
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
                        item_kind: Form(
                            Function,
                        ),
                    },
                    entity_path: Some(
                        AssociatedItemPath {
                            parent_path: TraitPath(`std::ops::Add`),
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 24,
                                    },
                                ),
                            ),
                        },
                    ),
                    ident_token: IdentifierToken {
                        ident: `add`,
                        token_idx: TokenIdx(
                            16,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
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
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                        TraitPath(`std::ops::Add`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..4,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)