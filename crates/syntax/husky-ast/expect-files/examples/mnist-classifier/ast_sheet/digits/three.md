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
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 35,
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
                                    value: 35,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `three_fermi_match`,
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
                    block: Form {
                        path: FugitivePath(
                            Id {
                                value: 42,
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
                                    value: 27,
                                },
                            ),
                        ),
                        variant: PubUnder {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                            lpar: LeftParenthesisToken(
                                TokenIdx(
                                    22,
                                ),
                            ),
                            scope: Super(
                                SuperToken {
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                            ),
                            rpar: RightParenthesisToken(
                                TokenIdx(
                                    24,
                                ),
                            ),
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `is_three`,
                        token_idx: TokenIdx(
                            26,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            27,
                        ),
                        drained: false,
                    },
                    block: Form {
                        path: FugitivePath(
                            Id {
                                value: 43,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    1..18,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 35,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `uparc`,
                        token_idx: TokenIdx(
                            156,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            157,
                        ),
                        drained: false,
                    },
                    block: Form {
                        path: FugitivePath(
                            Id {
                                value: 44,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    18..21,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 35,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `downarc`,
                        token_idx: TokenIdx(
                            190,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            191,
                        ),
                        drained: false,
                    },
                    block: Form {
                        path: FugitivePath(
                            Id {
                                value: 45,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    21..24,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 35,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `back`,
                        token_idx: TokenIdx(
                            224,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            225,
                        ),
                        drained: false,
                    },
                    block: Form {
                        path: FugitivePath(
                            Id {
                                value: 46,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    24..27,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            27..33,
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
                1..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..21,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..24,
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
                27..33,
            ),
        ],
    },
)