Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: ExpectNothing,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    error: ExpectNothing,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    error: ExcessiveIndent,
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
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..15,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        `mnist_classifier::line_segment_sketch::line_segment::LineSegment`,
                    ),
                    ident: `LineSegment`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            15..17,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `geom2d`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 1,
                },
            ],
        },
    },
)