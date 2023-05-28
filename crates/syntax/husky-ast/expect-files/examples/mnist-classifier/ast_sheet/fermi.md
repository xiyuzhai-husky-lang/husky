Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                8..9,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `norm`,
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            29,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    1..4,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `rel_norm`,
                        token_idx: TokenIdx(
                            66,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            67,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    5..8,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `angle_change_norm`,
                        token_idx: TokenIdx(
                            104,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            105,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    9..12,
                                ),
                            },
                        ),
                    },
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                15..17,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: None,
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    state_after_visibility_expr: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    4,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `FermiMatchResult`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            7,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 36,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    items: Some(
                        Type(
                            TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    12..15,
                                ),
                            },
                        ),
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    145,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `fermi_match`,
                        token_idx: TokenIdx(
                            147,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            148,
                        ),
                        drained: false,
                    },
                    block: Form {
                        path: FugitivePath(
                            Id {
                                value: 55,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    17..21,
                                ),
                            },
                        ),
                    },
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