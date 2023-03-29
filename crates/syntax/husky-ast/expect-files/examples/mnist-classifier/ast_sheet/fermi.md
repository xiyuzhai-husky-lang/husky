Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        6,
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
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        4..4,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        4..4,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        4..5,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        5..5,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        8..9,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        9..9,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        1..4,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        5..8,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `rel_norm`,
                        token_idx: TokenIdx(
                            65,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        66,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        9..12,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentToken {
                        ident: `angle_change_norm`,
                        token_idx: TokenIdx(
                            103,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        104,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        15..17,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        17..17,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `mnist_classifier::fermi`,
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
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
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        12..15,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        17..21,
                    ),
                    accessibility: Visibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `fermi_match`,
                        token_idx: TokenIdx(
                            146,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        147,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            21..25,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..1,
            ),
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..5,
            ),
            ArenaIdxRange(
                5..5,
            ),
            ArenaIdxRange(
                5..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..9,
            ),
            ArenaIdxRange(
                9..9,
            ),
            ArenaIdxRange(
                9..12,
            ),
            ArenaIdxRange(
                12..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..17,
            ),
            ArenaIdxRange(
                17..17,
            ),
            ArenaIdxRange(
                17..21,
            ),
            ArenaIdxRange(
                21..25,
            ),
        ],
    },
)