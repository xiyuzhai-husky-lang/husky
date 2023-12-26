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
                    8,
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
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    13,
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
                    30,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    27,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            16..19,
                        ),
                    },
                ),
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
            Ast::IfElseStmts {
                if_branch: 19,
                elif_branches: ArenaIdxRange(
                    20..20,
                ),
                else_branch: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    31,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    24,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            20..24,
                        ),
                    },
                ),
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
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    35,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    32,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            25..28,
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
                    23,
                ),
                body: None,
            },
            Ast::IfElseStmts {
                if_branch: 24,
                elif_branches: ArenaIdxRange(
                    25..25,
                ),
                else_branch: None,
            },
            Ast::IfElseStmts {
                if_branch: 28,
                elif_branches: ArenaIdxRange(
                    29..29,
                ),
                else_branch: None,
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
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    38,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    39,
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
                                value: 34,
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
                                value: 34,
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
                    ident: `simple_seven_match`,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                value: 34,
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
                    ident: `simple_leftdown_pattern`,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                2..5,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    7,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 34,
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
                    ident: `special_seven_match`,
                    token_idx: TokenIdx(
                        49,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        50,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                5..6,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    9,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 34,
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
                    ident: `leftupcc_pattern`,
                    token_idx: TokenIdx(
                        64,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        65,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                6..10,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    14,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 34,
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
                    ident: `leftdowncc_pattern`,
                    token_idx: TokenIdx(
                        107,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        108,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                10..16,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    21,
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
                                160,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                161,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    162,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                163,
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
                    ident: `is_seven`,
                    token_idx: TokenIdx(
                        165,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        166,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                29..37,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        37..44,
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
            2..2,
        ),
        ArenaIdxRange(
            2..2,
        ),
        ArenaIdxRange(
            2..5,
        ),
        ArenaIdxRange(
            5..5,
        ),
        ArenaIdxRange(
            5..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..19,
        ),
        ArenaIdxRange(
            20..20,
        ),
        ArenaIdxRange(
            20..24,
        ),
        ArenaIdxRange(
            25..25,
        ),
        ArenaIdxRange(
            25..25,
        ),
        ArenaIdxRange(
            25..25,
        ),
        ArenaIdxRange(
            25..28,
        ),
        ArenaIdxRange(
            29..29,
        ),
        ArenaIdxRange(
            29..29,
        ),
        ArenaIdxRange(
            29..29,
        ),
        ArenaIdxRange(
            29..29,
        ),
        ArenaIdxRange(
            29..37,
        ),
        ArenaIdxRange(
            37..44,
        ),
    ],
}