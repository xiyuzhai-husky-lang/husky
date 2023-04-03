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
                        11,
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
                                1..2,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 2,
                    elif_branches: ArenaIdxRange(
                        3..3,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                3..5,
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
                        8,
                    ),
                    body: None,
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
                        13,
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
                        15,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                13..14,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 14,
                    elif_branches: ArenaIdxRange(
                        15..15,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
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
                                    value: 25,
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
                                    value: 25,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `upper_mouth_match`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 10,
                            },
                        ),
                        body: Some(
                            FormBody {
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
                                    value: 24,
                                },
                            ),
                        ),
                        variant: PubUnder {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                            lpar: LeftParenthesisToken(
                                TokenIdx(
                                    19,
                                ),
                            ),
                            scope: Super(
                                SuperToken {
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                            ),
                            rpar: RightParenthesisToken(
                                TokenIdx(
                                    21,
                                ),
                            ),
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
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
                    saved_stream_state: TokenIdx(
                        24,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 11,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    6..13,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        14,
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
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `big_mouth`,
                        token_idx: TokenIdx(
                            87,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        88,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 12,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    15..17,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            17..21,
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
                1..1,
            ),
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..2,
            ),
            ArenaIdxRange(
                3..3,
            ),
            ArenaIdxRange(
                3..5,
            ),
            ArenaIdxRange(
                6..6,
            ),
            ArenaIdxRange(
                6..13,
            ),
            ArenaIdxRange(
                13..13,
            ),
            ArenaIdxRange(
                13..14,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..17,
            ),
            ArenaIdxRange(
                17..21,
            ),
        ],
    },
)