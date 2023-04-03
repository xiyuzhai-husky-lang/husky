Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            AssociatedFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `new`,
                        token_idx: TokenIdx(
                            86,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        87,
                    ),
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    0..2,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
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
                            124,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        125,
                    ),
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                        ),
                    },
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedIdent(
                            TokenIdx(
                                141,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        22,
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
                        32,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        33,
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
                        24,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        25,
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
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        29,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                8..13,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        35,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        37,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Memo,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `concave_components`,
                        token_idx: TokenIdx(
                            179,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        180,
                    ),
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Memo,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `bounding_box`,
                        token_idx: TokenIdx(
                            191,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        192,
                    ),
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    13..20,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        36,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            AssociatedFn,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `new`,
                        token_idx: TokenIdx(
                            317,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        318,
                    ),
                    block: AssociatedItem {
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    20..21,
                                ),
                            },
                        ),
                    },
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        38,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedIdent(
                            TokenIdx(
                                343,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        39,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
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
                        45,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        46,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        48,
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
                        50,
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
                        52,
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
                        60,
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
                        63,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        62,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                40..41,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        70,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        69,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                42..43,
                            ),
                        },
                    ),
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
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                44..45,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        77,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        76,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                46..47,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        79,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        78,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                48..49,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        74,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        75,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 47,
                    elif_branches: ArenaIdxRange(
                        48..48,
                    ),
                    else_branch: None,
                },
                Ast::IfElseStmts {
                    if_branch: 49,
                    elif_branches: ArenaIdxRange(
                        50..50,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        73,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                50..54,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        68,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 43,
                    elif_branches: ArenaIdxRange(
                        45..46,
                    ),
                    else_branch: None,
                },
                Ast::IfElseStmts {
                    if_branch: 54,
                    elif_branches: ArenaIdxRange(
                        55..55,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        80,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        81,
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
                        56,
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
                        58,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        59,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                38..40,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 41,
                    elif_branches: ArenaIdxRange(
                        42..42,
                    ),
                    else_branch: None,
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
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        66,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        67,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                55..60,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        82,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        83,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        89,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        90,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        92,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        91,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                74..75,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        100,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        99,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                76..77,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        102,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        101,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                78..79,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        107,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        106,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                80..81,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        109,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        108,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                82..83,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        104,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        105,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 81,
                    elif_branches: ArenaIdxRange(
                        82..82,
                    ),
                    else_branch: None,
                },
                Ast::IfElseStmts {
                    if_branch: 83,
                    elif_branches: ArenaIdxRange(
                        84..84,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        103,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                84..88,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        112,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        111,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                89..90,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 90,
                    elif_branches: ArenaIdxRange(
                        91..91,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        113,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        110,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                91..93,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        115,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        114,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                94..95,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        97,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        98,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 77,
                    elif_branches: ArenaIdxRange(
                        79..80,
                    ),
                    else_branch: None,
                },
                Ast::IfElseStmts {
                    if_branch: 88,
                    elif_branches: ArenaIdxRange(
                        89..89,
                    ),
                    else_branch: None,
                },
                Ast::IfElseStmts {
                    if_branch: 93,
                    elif_branches: ArenaIdxRange(
                        94..94,
                    ),
                    else_branch: Some(
                        95,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        117,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        116,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                101..102,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        119,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        118,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                103..104,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        85,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        86,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        87,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        88,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                72..74,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 75,
                    elif_branches: ArenaIdxRange(
                        76..76,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        93,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        94,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        95,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        96,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                96..101,
                            ),
                        },
                    ),
                },
                Ast::IfElseStmts {
                    if_branch: 102,
                    elif_branches: ArenaIdxRange(
                        103..103,
                    ),
                    else_branch: Some(
                        104,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        133,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        134,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        135,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        132,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                115..118,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        130,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        131,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 118,
                    elif_branches: ArenaIdxRange(
                        119..119,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        129,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                119..122,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        145,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        146,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        144,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                123..125,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        140,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        141,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        142,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        143,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 125,
                    elif_branches: ArenaIdxRange(
                        126..126,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        139,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                126..131,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        148,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        147,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                132..133,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        137,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        138,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 131,
                    elif_branches: ArenaIdxRange(
                        132..132,
                    ),
                    else_branch: Some(
                        133,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        149,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        136,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                134..138,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        126,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        127,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        128,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 122,
                    elif_branches: ArenaIdxRange(
                        123..123,
                    ),
                    else_branch: None,
                },
                Ast::IfElseStmts {
                    if_branch: 138,
                    elif_branches: ArenaIdxRange(
                        139..139,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        150,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        151,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        156,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        157,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        155,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                146..148,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        121,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        122,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        123,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        124,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        125,
                    ),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                139..146,
                            ),
                        },
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        152,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        153,
                    ),
                    body: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        154,
                    ),
                    body: None,
                },
                Ast::IfElseStmts {
                    if_branch: 148,
                    elif_branches: ArenaIdxRange(
                        149..149,
                    ),
                    else_branch: None,
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        158,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    ident_token: IdentToken {
                        ident: `concave_component`,
                        token_idx: TokenIdx(
                            1,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        2,
                    ),
                    block: Submodule {
                        path: ModulePath(
                            Id {
                                value: 38,
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    ident_token: IdentToken {
                        ident: `convex_component`,
                        token_idx: TokenIdx(
                            3,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        4,
                    ),
                    block: Submodule {
                        path: ModulePath(
                            Id {
                                value: 39,
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    ident_token: IdentToken {
                        ident: `convexity`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                    block: Submodule {
                        path: ModulePath(
                            Id {
                                value: 40,
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: Module,
                    ident_token: IdentToken {
                        ident: `line_segment`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        8,
                    ),
                    block: Submodule {
                        path: ModulePath(
                            Id {
                                value: 41,
                            },
                        ),
                    },
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                        },
                    },
                    state_after_visibility_expr: Some(
                        TokenIdx(
                            9,
                        ),
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    },
                    state_after_visibility_expr: Some(
                        TokenIdx(
                            14,
                        ),
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        6,
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
                    state_after_visibility_expr: Some(
                        TokenIdx(
                            19,
                        ),
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    state_after_visibility_expr: None,
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    state_after_visibility_expr: None,
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    state_after_visibility_expr: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    39,
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
                        ident: `LineSegmentStroke`,
                        token_idx: TokenIdx(
                            41,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        42,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 44,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    items: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                3..7,
                            ),
                        },
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
                                    159,
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
                        ident: `LineSegmentSketch`,
                        token_idx: TokenIdx(
                            161,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        162,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 45,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    items: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                21..26,
                            ),
                        },
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        40,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
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
                    ident_token: IdentToken {
                        ident: `go_right`,
                        token_idx: TokenIdx(
                            348,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        349,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 58,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    26..32,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        47,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
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
                    ident_token: IdentToken {
                        ident: `go_left`,
                        token_idx: TokenIdx(
                            445,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        446,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 59,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    32..38,
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
                                    value: 37,
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
                    ident_token: IdentToken {
                        ident: `extend_end`,
                        token_idx: TokenIdx(
                            542,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        543,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 60,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    60..72,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        84,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
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
                    ident_token: IdentToken {
                        ident: `extend_start`,
                        token_idx: TokenIdx(
                            775,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        776,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 61,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    105..115,
                                ),
                            },
                        ),
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        120,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 37,
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
                    ident_token: IdentToken {
                        ident: `find_line_segments`,
                        token_idx: TokenIdx(
                            1041,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        1042,
                    ),
                    block: Form {
                        path: FormPath(
                            Id {
                                value: 62,
                            },
                        ),
                        body: Some(
                            FormBody {
                                ast_idx_range: ArenaIdxRange(
                                    149..159,
                                ),
                            },
                        ),
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            159..178,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..3,
            ),
            ArenaIdxRange(
                3..7,
            ),
            ArenaIdxRange(
                7..7,
            ),
            ArenaIdxRange(
                7..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..13,
            ),
            ArenaIdxRange(
                13..13,
            ),
            ArenaIdxRange(
                13..20,
            ),
            ArenaIdxRange(
                20..20,
            ),
            ArenaIdxRange(
                20..21,
            ),
            ArenaIdxRange(
                21..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..26,
            ),
            ArenaIdxRange(
                26..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..32,
            ),
            ArenaIdxRange(
                32..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..38,
            ),
            ArenaIdxRange(
                38..40,
            ),
            ArenaIdxRange(
                40..40,
            ),
            ArenaIdxRange(
                40..41,
            ),
            ArenaIdxRange(
                42..42,
            ),
            ArenaIdxRange(
                42..42,
            ),
            ArenaIdxRange(
                42..42,
            ),
            ArenaIdxRange(
                42..42,
            ),
            ArenaIdxRange(
                42..42,
            ),
            ArenaIdxRange(
                42..43,
            ),
            ArenaIdxRange(
                44..44,
            ),
            ArenaIdxRange(
                44..45,
            ),
            ArenaIdxRange(
                46..46,
            ),
            ArenaIdxRange(
                46..46,
            ),
            ArenaIdxRange(
                46..46,
            ),
            ArenaIdxRange(
                46..47,
            ),
            ArenaIdxRange(
                48..48,
            ),
            ArenaIdxRange(
                48..49,
            ),
            ArenaIdxRange(
                50..54,
            ),
            ArenaIdxRange(
                55..55,
            ),
            ArenaIdxRange(
                55..55,
            ),
            ArenaIdxRange(
                55..60,
            ),
            ArenaIdxRange(
                60..60,
            ),
            ArenaIdxRange(
                60..60,
            ),
            ArenaIdxRange(
                60..72,
            ),
            ArenaIdxRange(
                72..72,
            ),
            ArenaIdxRange(
                72..72,
            ),
            ArenaIdxRange(
                72..72,
            ),
            ArenaIdxRange(
                72..72,
            ),
            ArenaIdxRange(
                72..72,
            ),
            ArenaIdxRange(
                72..74,
            ),
            ArenaIdxRange(
                74..74,
            ),
            ArenaIdxRange(
                74..75,
            ),
            ArenaIdxRange(
                76..76,
            ),
            ArenaIdxRange(
                76..76,
            ),
            ArenaIdxRange(
                76..76,
            ),
            ArenaIdxRange(
                76..76,
            ),
            ArenaIdxRange(
                76..76,
            ),
            ArenaIdxRange(
                76..76,
            ),
            ArenaIdxRange(
                76..77,
            ),
            ArenaIdxRange(
                78..78,
            ),
            ArenaIdxRange(
                78..79,
            ),
            ArenaIdxRange(
                80..80,
            ),
            ArenaIdxRange(
                80..80,
            ),
            ArenaIdxRange(
                80..80,
            ),
            ArenaIdxRange(
                80..81,
            ),
            ArenaIdxRange(
                82..82,
            ),
            ArenaIdxRange(
                82..83,
            ),
            ArenaIdxRange(
                84..88,
            ),
            ArenaIdxRange(
                89..89,
            ),
            ArenaIdxRange(
                89..90,
            ),
            ArenaIdxRange(
                91..91,
            ),
            ArenaIdxRange(
                91..93,
            ),
            ArenaIdxRange(
                94..94,
            ),
            ArenaIdxRange(
                94..95,
            ),
            ArenaIdxRange(
                96..101,
            ),
            ArenaIdxRange(
                101..101,
            ),
            ArenaIdxRange(
                101..102,
            ),
            ArenaIdxRange(
                103..103,
            ),
            ArenaIdxRange(
                103..104,
            ),
            ArenaIdxRange(
                105..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..115,
            ),
            ArenaIdxRange(
                115..118,
            ),
            ArenaIdxRange(
                119..122,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..123,
            ),
            ArenaIdxRange(
                123..125,
            ),
            ArenaIdxRange(
                126..131,
            ),
            ArenaIdxRange(
                132..132,
            ),
            ArenaIdxRange(
                132..133,
            ),
            ArenaIdxRange(
                134..134,
            ),
            ArenaIdxRange(
                134..138,
            ),
            ArenaIdxRange(
                139..139,
            ),
            ArenaIdxRange(
                139..139,
            ),
            ArenaIdxRange(
                139..146,
            ),
            ArenaIdxRange(
                146..146,
            ),
            ArenaIdxRange(
                146..146,
            ),
            ArenaIdxRange(
                146..146,
            ),
            ArenaIdxRange(
                146..146,
            ),
            ArenaIdxRange(
                146..146,
            ),
            ArenaIdxRange(
                146..148,
            ),
            ArenaIdxRange(
                149..149,
            ),
            ArenaIdxRange(
                149..159,
            ),
            ArenaIdxRange(
                159..178,
            ),
        ],
    },
)