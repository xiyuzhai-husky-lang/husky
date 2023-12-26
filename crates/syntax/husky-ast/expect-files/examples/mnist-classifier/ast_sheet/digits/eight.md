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
                    7,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    6,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            2..3,
                        ),
                    },
                ),
            },
            Ast::IfElseStmts {
                if_branch: 3,
                elif_branches: ArenaIdxRange(
                    4..4,
                ),
                else_branch: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    8,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            4..6,
                        ),
                    },
                ),
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    4,
                ),
                body: None,
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
                    9,
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
                    11,
                ),
                body: Some(
                    FugitiveBody {
                        ast_idx_range: ArenaIdxRange(
                            10..11,
                        ),
                    },
                ),
            },
            Ast::IfElseStmts {
                if_branch: 11,
                elif_branches: ArenaIdxRange(
                    12..12,
                ),
                else_branch: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    13,
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
                                value: 29,
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
                                value: 29,
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
                    ident: `upper_mouth_match`,
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
                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                value: 28,
                            },
                        ),
                    ),
                    variant: PubUnder {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                19,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                21,
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
                    ident: `is_eight`,
                    token_idx: TokenIdx(
                        23,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        24,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                7..10,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    10,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 29,
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
                    ident: `big_mouth`,
                    token_idx: TokenIdx(
                        73,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        74,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                12..14,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        14..18,
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
            2..3,
        ),
        ArenaIdxRange(
            4..4,
        ),
        ArenaIdxRange(
            4..6,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..10,
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
            12..14,
        ),
        ArenaIdxRange(
            14..18,
        ),
    ],
}