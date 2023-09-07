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
                        8,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..3,
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
                    if_branch: 3,
                    elif_branches: ArenaIdxRange(
                        4..4,
                    ),
                    else_branch: None,
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
                        11,
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
                                10..11,
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
                        22,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
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
                                    value: 46,
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
                                    5,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `connected_components`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            8,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 64,
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
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `major_connected_component`,
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
                                value: 65,
                            },
                        ),
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
                        12,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    68,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        token_idx: TokenIdx(
                            70,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            71,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 66,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    11..14,
                                ),
                            },
                        ),
                    },
                },
                Ast::Identifiable {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    102,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `major_raw_contours`,
                        token_idx: TokenIdx(
                            104,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            105,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 67,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                        ),
                    },
                },
                Ast::Identifiable {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    114,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `major_raw_contour`,
                        token_idx: TokenIdx(
                            116,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            117,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 68,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    15..16,
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
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    127,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `major_line_segment_sketch`,
                        token_idx: TokenIdx(
                            129,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            130,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 69,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    16..17,
                                ),
                            },
                        ),
                    },
                },
                Ast::Identifiable {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    137,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Val,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `major_concave_components`,
                        token_idx: TokenIdx(
                            139,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            140,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 70,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    17..18,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            18..26,
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
                1..3,
            ),
            ArenaIdxRange(
                4..6,
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
                10..11,
            ),
            ArenaIdxRange(
                11..11,
            ),
            ArenaIdxRange(
                11..14,
            ),
            ArenaIdxRange(
                14..14,
            ),
            ArenaIdxRange(
                14..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..16,
            ),
            ArenaIdxRange(
                16..16,
            ),
            ArenaIdxRange(
                16..17,
            ),
            ArenaIdxRange(
                17..17,
            ),
            ArenaIdxRange(
                17..18,
            ),
            ArenaIdxRange(
                18..26,
            ),
        ],
    },
)