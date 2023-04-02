Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        1..3,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        1..1,
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
                        4,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        4..6,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        6..6,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        10..11,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        11..11,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        14..14,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        16..16,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        17..17,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 42,
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
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    4,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `connected_components`,
                        token_idx: TokenIdx(
                            6,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        7,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        0..1,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `major_connected_component`,
                        token_idx: TokenIdx(
                            20,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        21,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        6..10,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    70,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        token_idx: TokenIdx(
                            72,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        73,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        11..14,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    105,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `major_raw_contours`,
                        token_idx: TokenIdx(
                            107,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        108,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        14..15,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    118,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `major_raw_contour`,
                        token_idx: TokenIdx(
                            120,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        121,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        15..16,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    132,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `major_line_segment_sketch`,
                        token_idx: TokenIdx(
                            134,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        135,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        16..17,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    143,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Val,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `major_concave_components`,
                        token_idx: TokenIdx(
                            145,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        146,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        17..18,
                    ),
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