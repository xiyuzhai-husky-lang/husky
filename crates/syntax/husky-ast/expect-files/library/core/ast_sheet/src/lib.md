```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                1,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `array`,
                    token_idx: TokenIdx(
                        3,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        4,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::array),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `backend`,
                    token_idx: TokenIdx(
                        6,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        7,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::backend),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `basic`,
                    token_idx: TokenIdx(
                        9,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        10,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::basic),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `clone`,
                    token_idx: TokenIdx(
                        12,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        13,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::clone),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                13,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `cmp`,
                    token_idx: TokenIdx(
                        15,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        16,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::cmp),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 5,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `default`,
                    token_idx: TokenIdx(
                        18,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        19,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::default),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 6,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `frontend`,
                    token_idx: TokenIdx(
                        21,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        22,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::frontend),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 7,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                22,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `fmt`,
                    token_idx: TokenIdx(
                        24,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        25,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::fmt),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 8,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                25,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `logic`,
                    token_idx: TokenIdx(
                        27,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        28,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::logic),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 9,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                28,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `marker`,
                    token_idx: TokenIdx(
                        30,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        31,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::marker),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 10,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                31,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `mem`,
                    token_idx: TokenIdx(
                        33,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        34,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::mem),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 11,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                34,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `num`,
                    token_idx: TokenIdx(
                        36,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        37,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::num),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 12,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                37,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `ops`,
                    token_idx: TokenIdx(
                        39,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        40,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::ops),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 13,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                40,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `option`,
                    token_idx: TokenIdx(
                        42,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        43,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::option),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 14,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                43,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `prelude`,
                    token_idx: TokenIdx(
                        45,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        46,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::prelude),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 15,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                46,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `raw_bits`,
                    token_idx: TokenIdx(
                        48,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        49,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::raw_bits),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 16,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                49,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `result`,
                    token_idx: TokenIdx(
                        51,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        52,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::result),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 17,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                52,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `task`,
                    token_idx: TokenIdx(
                        54,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        55,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::task),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 18,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                55,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `vec`,
                    token_idx: TokenIdx(
                        57,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        58,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::vec),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 19,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                58,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `slice`,
                    token_idx: TokenIdx(
                        60,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        61,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::slice),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 20,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                61,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `str`,
                    token_idx: TokenIdx(
                        63,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        64,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::str),
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 21,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                64,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `visual`,
                    token_idx: TokenIdx(
                        66,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        67,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(`core::visual),
                },
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 22,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`core`),
                    ),
                },
                state_after_visibility_expr: None,
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        0..23,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..23,
        ),
    ],
}
```