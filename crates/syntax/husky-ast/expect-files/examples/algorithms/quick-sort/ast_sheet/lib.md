Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                2..5,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 5,
                    elif_branches: ArenaIdxRange(
                        6..6,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        18,
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
                        19,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                9..10,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                11..12,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                7..8,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                8..9,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 10,
                    elif_branches: ArenaIdxRange(
                        11..11,
                    ),
                    else_branch: Some(
                        12,
                    ),
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
                        12,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                13..18,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        32,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        33,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        34,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    0,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `quick_sort`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            3,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 1,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    0..2,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 25,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `quick_sort_aux`,
                        token_idx: TokenIdx(
                            41,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            42,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 2,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    6..7,
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
                                    value: 25,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `partition`,
                        token_idx: TokenIdx(
                            102,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            103,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 3,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    18..24,
                                ),
                            },
                        ),
                    },
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    ident: `test`,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 25,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `quick_sort_works_for_integers`,
                        token_idx: TokenIdx(
                            227,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            228,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 4,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    24..27,
                                ),
                            },
                        ),
                    },
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        30,
                    ),
                    ident: `test`,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        31,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 25,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `quick_sort_works_for_strs`,
                        token_idx: TokenIdx(
                            289,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            290,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 5,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    27..30,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            30..37,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..5,
            ),
            ArenaIdxRange(
                6..7,
            ),
            ArenaIdxRange(
                7..7,
            ),
            ArenaIdxRange(
                7..7,
            ),
            ArenaIdxRange(
                7..7,
            ),
            ArenaIdxRange(
                7..7,
            ),
            ArenaIdxRange(
                7..7,
            ),
            ArenaIdxRange(
                7..8,
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
                9..10,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..12,
            ),
            ArenaIdxRange(
                13..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..24,
            ),
            ArenaIdxRange(
                24..24,
            ),
            ArenaIdxRange(
                24..24,
            ),
            ArenaIdxRange(
                24..24,
            ),
            ArenaIdxRange(
                24..27,
            ),
            ArenaIdxRange(
                27..27,
            ),
            ArenaIdxRange(
                27..27,
            ),
            ArenaIdxRange(
                27..27,
            ),
            ArenaIdxRange(
                27..30,
            ),
            ArenaIdxRange(
                30..37,
            ),
        ],
    },
)