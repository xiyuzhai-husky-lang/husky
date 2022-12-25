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
                                value: 5,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        item_kind: Type(
                            Form,
                        ),
                    },
                    entity_path: Some(
                        AssociatedItem(
                            AssociatedItemPath {
                                parent_path: Connected(
                                    ConnectedModuleItemPath {
                                        module_path: `core::ops`,
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ident: `Output`,
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
                                value: 5,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        item_kind: Form,
                    },
                    entity_path: Some(
                        AssociatedItem(
                            AssociatedItemPath {
                                parent_path: Connected(
                                    ConnectedModuleItemPath {
                                        module_path: `core::ops`,
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 24,
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ident: `add`,
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
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        item_kind: Trait,
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `core::ops`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                    ident: `Add`,
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
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