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
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: ExpectNothing,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
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
                                value: 28,
                            },
                        ),
                    ),
                    use_expr_idx: 6,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    use_expr_idx: 10,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    use_expr_idx: 13,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..3,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `ConcaveComponent`,
                        token_idx: TokenIdx(
                            29,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        30,
                    ),
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
            ],
        },
        top_level_asts: ArenaIdxRange(
            3..16,
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
                    parent: `line_segment`,
                    child: 3,
                },
                ScopeResolution {
                    parent: `line_segment_sketch`,
                    child: 4,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 5,
                },
                All,
                ScopeResolution {
                    parent: `convexity`,
                    child: 7,
                },
                ScopeResolution {
                    parent: `line_segment_sketch`,
                    child: 8,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 9,
                },
                All,
                ScopeResolution {
                    parent: `geom2d`,
                    child: 11,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 12,
                },
            ],
        },
    },
)