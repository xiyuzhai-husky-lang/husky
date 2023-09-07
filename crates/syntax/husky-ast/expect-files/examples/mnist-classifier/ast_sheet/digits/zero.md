Ok(
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
                        8,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                3..10,
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
                Ast::IfElseStmts {
                    if_branch: 10,
                    elif_branches: ArenaIdxRange(
                        11..11,
                    ),
                    else_branch: None,
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
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 38,
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
                                    value: 38,
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
                        ident: `open_one_match`,
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
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 53,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    0..1,
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
                                    value: 38,
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
                        ident: `almost_closed`,
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
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 54,
                            },
                        ),
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
                        6,
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
                                    45,
                                ),
                            },
                            lpar: LparToken(
                                TokenIdx(
                                    46,
                                ),
                            ),
                            visibility: Super(
                                SuperToken {
                                    token_idx: TokenIdx(
                                        47,
                                    ),
                                },
                            ),
                            rpar: RparToken(
                                TokenIdx(
                                    48,
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
                        ident: `is_zero`,
                        token_idx: TokenIdx(
                            50,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            51,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 55,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    11..25,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            25..29,
        ),
        siblings: [
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
                3..10,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..25,
            ),
            ArenaIdxRange(
                25..29,
            ),
        ],
    },
)