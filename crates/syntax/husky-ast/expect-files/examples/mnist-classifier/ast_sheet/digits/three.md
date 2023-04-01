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
                        1..1,
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
                        1..1,
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
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    body: ArenaIdxRange(
                        1..1,
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
                        15,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        1..1,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    body: ArenaIdxRange(
                        18..18,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    body: ArenaIdxRange(
                        21..21,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        27,
                    ),
                    body: ArenaIdxRange(
                        21..21,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        28,
                    ),
                    body: ArenaIdxRange(
                        21..21,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        30,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        31,
                    ),
                    body: ArenaIdxRange(
                        24..24,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        32,
                    ),
                    body: ArenaIdxRange(
                        24..24,
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
                                    value: 32,
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
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 32,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Var,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Var`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `three_fermi_match`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        6,
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
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 24,
                                },
                            ),
                        ),
                        variant: PubUnder {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                            lpar: LeftParenthesisToken(
                                TokenIdx(
                                    23,
                                ),
                            ),
                            scope: Super(
                                SuperToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            rpar: RightParenthesisToken(
                                TokenIdx(
                                    25,
                                ),
                            ),
                        },
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Var,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::is_three`, `Var`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `is_three`,
                        token_idx: TokenIdx(
                            27,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        28,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        1..18,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 32,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::uparc`, `Fn`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `uparc`,
                        token_idx: TokenIdx(
                            158,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        159,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        18..21,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 32,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::downarc`, `Fn`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `downarc`,
                        token_idx: TokenIdx(
                            192,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        193,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        21..24,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 32,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Fn,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::back`, `Fn`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `back`,
                        token_idx: TokenIdx(
                            226,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        227,
                    ),
                    body_kind: Block,
                    body: ArenaIdxRange(
                        24..27,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            27..33,
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
                1..1,
            ),
            ArenaIdxRange(
                1..1,
            ),
            ArenaIdxRange(
                1..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..18,
            ),
            ArenaIdxRange(
                18..21,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..21,
            ),
            ArenaIdxRange(
                21..24,
            ),
            ArenaIdxRange(
                24..24,
            ),
            ArenaIdxRange(
                24..24,
            ),
            ArenaIdxRange(
                24..24,
            ),
            ArenaIdxRange(
                24..27,
            ),
            ArenaIdxRange(
                27..33,
            ),
        ],
    },
)