Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        1..3,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                IfElseStmts {
                    if_stmt: 3,
                    elif_stmts: ArenaIdxRange(
                        4..4,
                    ),
                    else_stmt: None,
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        4..6,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        6..6,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        10..11,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        11..11,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        14..14,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: ArenaIdxRange(
                        16..16,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: ArenaIdxRange(
                        17..17,
                    ),
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 8,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 12,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::connected_components`,
                    ),
                    ident: `connected_components`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        6..10,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::major_connected_component`,
                    ),
                    ident: `major_connected_component`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        11..14,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::ignored_connected_components_row_span_sum_sum`,
                    ),
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        14..15,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::major_raw_contours`,
                    ),
                    ident: `major_raw_contours`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Comment {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        15..16,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::major_raw_contour`,
                    ),
                    ident: `major_raw_contour`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    body: ArenaIdxRange(
                        16..17,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::major_line_segment_sketch`,
                    ),
                    ident: `major_line_segment_sketch`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: ArenaIdxRange(
                        17..18,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::major::major_concave_components`,
                    ),
                    ident: `major_concave_components`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            18..30,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `connected_component`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 1,
                },
                All,
                ScopeResolution {
                    parent: `raw_contour`,
                    child: 3,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 4,
                },
                All,
                ScopeResolution {
                    parent: `line_segment_sketch`,
                    child: 6,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 7,
                },
                All,
                ScopeResolution {
                    parent: `concave_component`,
                    child: 9,
                },
                ScopeResolution {
                    parent: `line_segment_sketch`,
                    child: 10,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 11,
                },
            ],
        },
    },
)