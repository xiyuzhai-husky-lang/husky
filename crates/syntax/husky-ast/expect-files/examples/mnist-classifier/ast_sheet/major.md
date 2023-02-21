Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        1..3,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
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
                        8,
                    ),
                    body: ArenaIdxRange(
                        1..1,
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
                        4..6,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: ArenaIdxRange(
                        6..6,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        10..10,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        10..11,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        11..11,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        14..14,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        15..15,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: ArenaIdxRange(
                        16..16,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: ArenaIdxRange(
                        17..17,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `connected_components`,
                        token_idx: TokenIdx(
                            32,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        33,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        6..10,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `major_connected_component`,
                        token_idx: TokenIdx(
                            44,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        45,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        11..14,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `ignored_connected_components_row_span_sum_sum`,
                        token_idx: TokenIdx(
                            94,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        95,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        14..15,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `major_raw_contours`,
                        token_idx: TokenIdx(
                            128,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        129,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        15..16,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `major_raw_contour`,
                        token_idx: TokenIdx(
                            139,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        140,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    body: ArenaIdxRange(
                        16..17,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `major_line_segment_sketch`,
                        token_idx: TokenIdx(
                            151,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        152,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: ArenaIdxRange(
                        17..18,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `major_concave_components`,
                        token_idx: TokenIdx(
                            160,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        161,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            18..30,
        ),
    },
)