Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
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
                        5,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        2..5,
                    ),
                },
                IfElseStmts {
                    if_stmt: 5,
                    elif_stmts: ArenaIdxRange(
                        6..6,
                    ),
                    else_stmt: None,
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        9..9,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        9..10,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        22,
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
                        11..12,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        7..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        8..8,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        8..9,
                    ),
                },
                IfElseStmts {
                    if_stmt: 10,
                    elif_stmts: ArenaIdxRange(
                        11..11,
                    ),
                    else_stmt: Some(
                        12,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    body: ArenaIdxRange(
                        7..7,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        13..18,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        32,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        33,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        34,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: Public,
                    entity_kind: ModuleItem {
                        item_kind: Form,
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `quick_sort`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 35,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                    ident: `quick_sort`,
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 3,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        6..7,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form,
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `quick_sort`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 40,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                    ident: `quick_sort_aux`,
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        18..24,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form,
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `quick_sort`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 45,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                    ident: `partition`,
                    is_generic: true,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: ArenaIdxRange(
                        24..27,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form,
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `quick_sort`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                    ident: `quick_sort_works_for_integers`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        30,
                    ),
                    body: ArenaIdxRange(
                        27..27,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        31,
                    ),
                    body: ArenaIdxRange(
                        27..30,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        item_kind: Form,
                        connection: Connected,
                    },
                    entity_path: Some(
                         ModuleItem(
                            Connected(
                                ConnectedModuleItemPath {
                                    module_path: `quick_sort`,
                                    ident: Identifier(
                                        Word(
                                            Id {
                                                value: 55,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                    ident: `quick_sort_works_for_strs`,
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIterState {
                        next_relative: 2,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            30..37,
        ),
        use_expr_arena: Arena {
            data: [],
        },
    },
)