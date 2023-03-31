Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    0,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::basic`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `basic`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::default`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `default`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::logic`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `logic`,
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        9,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::mem`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `mem`,
                        token_idx: TokenIdx(
                            11,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        12,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    12,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::num`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `num`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        15,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
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
                                    15,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::ops`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        18,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
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
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::prelude`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `prelude`,
                        token_idx: TokenIdx(
                            20,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        21,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::raw_bits`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `raw_bits`,
                        token_idx: TokenIdx(
                            23,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        24,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::fmt`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `fmt`,
                        token_idx: TokenIdx(
                            26,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        27,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::clone`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `clone`,
                        token_idx: TokenIdx(
                            29,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        30,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
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
                                    30,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::marker`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `marker`,
                        token_idx: TokenIdx(
                            32,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        33,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::list`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `list`,
                        token_idx: TokenIdx(
                            35,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        36,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::cmp`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `cmp`,
                        token_idx: TokenIdx(
                            38,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        39,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        13,
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
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::str`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `str`,
                        token_idx: TokenIdx(
                            41,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        42,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    42,
                                ),
                            },
                        },
                    },
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `core::option`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `option`,
                        token_idx: TokenIdx(
                            44,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        45,
                    ),
                    body_kind: None,
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..15,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..15,
            ),
        ],
    },
)