Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `visualize`,
                        token_idx: TokenIdx(
                            53,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            54,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                        ),
                    },
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        11,
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
                        23,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
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
                        20,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 5,
                    elif_branches: ArenaIdxRange(
                        6..6,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
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
                        19,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                6..9,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        30,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        31,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                15..18,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        32,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        40,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        41,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        42,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        43,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        44,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        34,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        35,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        36,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        37,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        38,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        39,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                22..27,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        45,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        47,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        49,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        51,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        53,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        55,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        57,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        59,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `norm`,
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
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        12,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `rel_norm`,
                        token_idx: TokenIdx(
                            78,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            79,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        14,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `hausdorff_norm`,
                        token_idx: TokenIdx(
                            96,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            97,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    9..15,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        25,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `angle_change`,
                        token_idx: TokenIdx(
                            184,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            185,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    18..22,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        33,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `bounding_box`,
                        token_idx: TokenIdx(
                            255,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            256,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    27..34,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        46,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MemoizedField,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `relative_bounding_box`,
                        token_idx: TokenIdx(
                            385,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            386,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    34..35,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        48,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `line_segment`,
                        token_idx: TokenIdx(
                            402,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            403,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    35..36,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        50,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `start`,
                        token_idx: TokenIdx(
                            441,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            442,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    36..37,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        52,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `end`,
                        token_idx: TokenIdx(
                            462,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            463,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    37..38,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        54,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `displacement`,
                        token_idx: TokenIdx(
                            483,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            484,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    38..39,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        56,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `start_tangent`,
                        token_idx: TokenIdx(
                            499,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            500,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    39..40,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        58,
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
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `end_tangent`,
                        token_idx: TokenIdx(
                            518,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            519,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    40..41,
                                ),
                            },
                        ),
                    },
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        66,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        70,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        72,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        71,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                55..56,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        69,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                54..55,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 56,
                    elif_branches: ArenaIdxRange(
                        57..57,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        73,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        74,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        61,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        62,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        63,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        64,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        65,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                53..54,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        67,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        68,
                    ),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                57..61,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        75,
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
                                    value: 42,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    state_after_visibility_expr: None,
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        1,
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
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        2,
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
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        3,
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
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        4,
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
                        5,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `ConcaveComponent`,
                        token_idx: TokenIdx(
                            34,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            35,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 46,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                        ),
                    ),
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    items: Some(
                        Type(
                            TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    41..53,
                                ),
                            },
                        ),
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        60,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    536,
                                ),
                            },
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Fugitive(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `find_concave_components`,
                        token_idx: TokenIdx(
                            538,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            539,
                        ),
                        drained: false,
                    },
                    block: Fugitive {
                        path: FugitivePath(
                            Id {
                                value: 56,
                            },
                        ),
                        body: Some(
                            FugitiveBody {
                                ast_idx_range: ArenaIdxRange(
                                    61..69,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            69..78,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..1,
            ),
            ArenaIdxRange(
                1..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..3,
            ),
            ArenaIdxRange(
                3..3,
            ),
            ArenaIdxRange(
                3..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..4,
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
                6..9,
            ),
            ArenaIdxRange(
                9..9,
            ),
            ArenaIdxRange(
                9..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..15,
            ),
            ArenaIdxRange(
                15..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..22,
            ),
            ArenaIdxRange(
                22..27,
            ),
            ArenaIdxRange(
                27..27,
            ),
            ArenaIdxRange(
                27..34,
            ),
            ArenaIdxRange(
                34..34,
            ),
            ArenaIdxRange(
                34..35,
            ),
            ArenaIdxRange(
                35..35,
            ),
            ArenaIdxRange(
                35..36,
            ),
            ArenaIdxRange(
                36..36,
            ),
            ArenaIdxRange(
                36..37,
            ),
            ArenaIdxRange(
                37..37,
            ),
            ArenaIdxRange(
                37..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..39,
            ),
            ArenaIdxRange(
                39..39,
            ),
            ArenaIdxRange(
                39..40,
            ),
            ArenaIdxRange(
                40..40,
            ),
            ArenaIdxRange(
                40..41,
            ),
            ArenaIdxRange(
                41..53,
            ),
            ArenaIdxRange(
                53..53,
            ),
            ArenaIdxRange(
                53..53,
            ),
            ArenaIdxRange(
                53..53,
            ),
            ArenaIdxRange(
                53..53,
            ),
            ArenaIdxRange(
                53..53,
            ),
            ArenaIdxRange(
                53..54,
            ),
            ArenaIdxRange(
                54..54,
            ),
            ArenaIdxRange(
                54..54,
            ),
            ArenaIdxRange(
                54..55,
            ),
            ArenaIdxRange(
                55..55,
            ),
            ArenaIdxRange(
                55..56,
            ),
            ArenaIdxRange(
                57..57,
            ),
            ArenaIdxRange(
                57..57,
            ),
            ArenaIdxRange(
                57..61,
            ),
            ArenaIdxRange(
                61..61,
            ),
            ArenaIdxRange(
                61..69,
            ),
            ArenaIdxRange(
                69..78,
            ),
        ],
    },
)