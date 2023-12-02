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
                            3..6,
                        ),
                    },
                ),
            },
            Ast::IfElseStmts {
                if_branch: 6,
                elif_branches: ArenaIdxRange(
                    7..7,
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
                            10..11,
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
                            12..13,
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
                            8..9,
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
                            9..10,
                        ),
                    },
                ),
            },
            Ast::IfElseStmts {
                if_branch: 11,
                elif_branches: ArenaIdxRange(
                    12..12,
                ),
                else_branch: Some(
                    13,
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
                            14..19,
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
                    30,
                ),
                body: None,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                1,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        FunctionFn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort`,
                    token_idx: TokenIdx(
                        3,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        4,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`quick_sort::quick_sort`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..3,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
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
                        FunctionFn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort_aux`,
                    token_idx: TokenIdx(
                        43,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        44,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`quick_sort::quick_sort_aux`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                7..8,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
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
                        FunctionFn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `partition`,
                    token_idx: TokenIdx(
                        105,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        106,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`quick_sort::partition`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                19..25,
                            ),
                        },
                    ),
                },
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    25,
                ),
                ident: `test`,
            },
            Ast::Identifiable {
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
                        FunctionFn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort_works_for_integers`,
                    token_idx: TokenIdx(
                        231,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        232,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                25..26,
                            ),
                        },
                    ),
                },
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    28,
                ),
                ident: `test`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    29,
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
                        FunctionFn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `quick_sort_works_for_strs`,
                    token_idx: TokenIdx(
                        268,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        269,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                26..27,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        27..34,
    ),
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..6,
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
            8..8,
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
            9..9,
        ),
        ArenaIdxRange(
            9..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..11,
        ),
        ArenaIdxRange(
            12..12,
        ),
        ArenaIdxRange(
            12..13,
        ),
        ArenaIdxRange(
            14..19,
        ),
        ArenaIdxRange(
            19..19,
        ),
        ArenaIdxRange(
            19..19,
        ),
        ArenaIdxRange(
            19..25,
        ),
        ArenaIdxRange(
            25..25,
        ),
        ArenaIdxRange(
            25..26,
        ),
        ArenaIdxRange(
            26..26,
        ),
        ArenaIdxRange(
            26..27,
        ),
        ArenaIdxRange(
            27..34,
        ),
    ],
}