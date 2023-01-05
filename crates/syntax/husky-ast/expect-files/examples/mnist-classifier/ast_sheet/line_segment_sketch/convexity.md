Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        2..5,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
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
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                IfElseStmts {
                    if_stmt: 5,
                    elif_stmts: ArenaIdxRange(
                        6..6,
                    ),
                    else_stmt: None,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    error: StandaloneElse,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    error: ExcessiveIndent,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
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
                                value: 30,
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
                                value: 30,
                            },
                        ),
                    ),
                    use_expr_idx: 8,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        6..19,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Function,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `is_convex`,
                        token_idx: TokenIdx(
                            19,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        20,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            19..23,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `line_segment_sketch`,
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
                    parent: `geom2d`,
                    child: 6,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 7,
                },
            ],
        },
    },
)