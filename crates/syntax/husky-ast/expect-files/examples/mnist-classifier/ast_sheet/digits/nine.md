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
                        15,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                2..16,
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
                Ast::IfElseStmts {
                    if_branch: 16,
                    elif_branches: ArenaIdxRange(
                        17..17,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        30,
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
                                    value: 32,
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
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            6,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 21,
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
                Ast::Defn {
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
                            18,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            19,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 22,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
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
                                    30,
                                ),
                            },
                            lpar: LparToken(
                                TokenIdx(
                                    31,
                                ),
                            ),
                            visibility: Super(
                                SuperToken {
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                            ),
                            rpar: RparToken(
                                TokenIdx(
                                    33,
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
                            35,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            36,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 23,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    17..28,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        31,
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
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `downmost`,
                        token_idx: TokenIdx(
                            214,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            215,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 24,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    28..31,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        35,
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
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `big_cc`,
                        token_idx: TokenIdx(
                            243,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            244,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 25,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    31..35,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            35..41,
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
                2..2,
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
                2..2,
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
                2..2,
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
                2..2,
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
                2..2,
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
                2..16,
            ),
            ArenaIdxRange(
                17..17,
            ),
            ArenaIdxRange(
                17..28,
            ),
            ArenaIdxRange(
                28..28,
            ),
            ArenaIdxRange(
                28..28,
            ),
            ArenaIdxRange(
                28..28,
            ),
            ArenaIdxRange(
                28..31,
            ),
            ArenaIdxRange(
                31..31,
            ),
            ArenaIdxRange(
                31..31,
            ),
            ArenaIdxRange(
                31..31,
            ),
            ArenaIdxRange(
                31..31,
            ),
            ArenaIdxRange(
                31..35,
            ),
            ArenaIdxRange(
                35..41,
            ),
        ],
    },
)