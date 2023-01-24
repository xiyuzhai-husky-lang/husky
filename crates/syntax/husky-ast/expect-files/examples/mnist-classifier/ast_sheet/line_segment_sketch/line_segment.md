Ok(
    AstSheet {
        arena: Arena {
            data: [
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
                        9,
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
                        1..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        3..3,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        3..4,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        5..5,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        5..6,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        3..3,
                    ),
                },
                IfElseStmts {
                    if_stmt: 4,
                    elif_stmts: ArenaIdxRange(
                        5..5,
                    ),
                    else_stmt: Some(
                        6,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        7..9,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                IfElseStmts {
                    if_stmt: 2,
                    elif_stmts: ArenaIdxRange(
                        3..3,
                    ),
                    else_stmt: Some(
                        9,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 37,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Method,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `displacement`,
                        token_idx: TokenIdx(
                            23,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        24,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        10..13,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 37,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Method,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `dist_to_point`,
                        token_idx: TokenIdx(
                            40,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        41,
                    ),
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `LineSegment`,
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        9,
                    ),
                },
                Impl {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        13..15,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            15..18,
        ),
    },
)