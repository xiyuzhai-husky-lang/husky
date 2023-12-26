AstSheet {
    ast_arena: Arena {
        data: [
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    2,
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
                    14,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    15,
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
                    17,
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
                body: None,
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
                    24,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    25,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    26,
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
                    13,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            3..17,
                        ),
                    },
                ),
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
                    8,
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
                body: None,
            },
            Ast::IfElseStmts {
                if_branch: 17,
                elif_branches: ArenaIdxRange(
                    18..18,
                ),
                else_branch: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    28,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    30,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    31,
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
                    34,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    35,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    36,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    37,
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
                                value: 32,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                state_after_visibility_expr: None,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        Val,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `nine_match`,
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
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
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
                                value: 32,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        Val,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `nine_match_refine`,
                    token_idx: TokenIdx(
                        19,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        20,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    variant: PubUnder {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                31,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                32,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                34,
                            ),
                        ),
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        Val,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `is_nine`,
                    token_idx: TokenIdx(
                        36,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        37,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                18..27,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    29,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 32,
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
                    ident: `downmost`,
                    token_idx: TokenIdx(
                        216,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        217,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                27..30,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    33,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 32,
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
                    ident: `big_cc`,
                    token_idx: TokenIdx(
                        245,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        246,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                30..34,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        34..40,
    ),
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            2..2,
        ),
        ArenaIdxRange(
            2..3,
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
            3..3,
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
            3..3,
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
            3..3,
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
            3..3,
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
            3..3,
        ),
        ArenaIdxRange(
            3..3,
        ),
        ArenaIdxRange(
            3..17,
        ),
        ArenaIdxRange(
            18..18,
        ),
        ArenaIdxRange(
            18..27,
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
            30..30,
        ),
        ArenaIdxRange(
            30..30,
        ),
        ArenaIdxRange(
            30..30,
        ),
        ArenaIdxRange(
            30..30,
        ),
        ArenaIdxRange(
            30..34,
        ),
        ArenaIdxRange(
            34..40,
        ),
    ],
}