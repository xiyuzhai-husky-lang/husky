Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        4..4,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        4..4,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        4..5,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        5..5,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        8..9,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        9..9,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        1..4,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeImplItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `norm`,
                        token_idx: TokenIdx(
                            27,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        28,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        5..8,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeImplItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `rel_norm`,
                        token_idx: TokenIdx(
                            61,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        62,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        9..12,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeImplItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `angle_change_norm`,
                        token_idx: TokenIdx(
                            95,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        96,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        15..17,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        17..17,
                    ),
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    use_expr_idx: 1,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `FermiMatchResult`,
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
                Impl {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        12..15,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        17..21,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Function,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `fermi_match`,
                        token_idx: TokenIdx(
                            133,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        134,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            21..25,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `crate`,
                    child: 0,
                },
            ],
        },
    },
)