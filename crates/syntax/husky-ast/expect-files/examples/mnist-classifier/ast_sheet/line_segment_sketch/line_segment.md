Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        4,
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
                        8,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                2..3,
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
                        12,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
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
                                6..7,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 5,
                    elif_branches: ArenaIdxRange(
                        6..6,
                    ),
                    else_branch: Some(
                        7,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                8..10,
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
                Ast::IfElseStmts {
                    if_branch: 3,
                    elif_branches: ArenaIdxRange(
                        4..4,
                    ),
                    else_branch: Some(
                        10,
                    ),
                },
                Ast::Identifiable {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `displacement`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            25,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
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
                        5,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `dist_to_point`,
                        token_idx: TokenIdx(
                            41,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            42,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    11..14,
                                ),
                            },
                        ),
                    },
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 45,
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
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `LineSegment`,
                        token_idx: TokenIdx(
                            9,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            10,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 49,
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
                                    14..16,
                                ),
                            },
                        ),
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            16..19,
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
                2..3,
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
                6..6,
            ),
            ArenaIdxRange(
                6..7,
            ),
            ArenaIdxRange(
                8..10,
            ),
            ArenaIdxRange(
                11..14,
            ),
            ArenaIdxRange(
                14..16,
            ),
            ArenaIdxRange(
                16..19,
            ),
        ],
    },
)