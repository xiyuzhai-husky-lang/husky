AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
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
                item_kind: Module,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::array`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `basic`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::basic`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `clone`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::clone`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `cmp`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::cmp`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `default`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::default`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        6,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `fmt`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::fmt`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        7,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `logic`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::logic`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        8,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `marker`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::marker`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        9,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `mem`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::mem`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        10,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `num`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::num`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        11,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `ops`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::ops`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        12,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `option`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::option`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        13,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `prelude`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::prelude`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        14,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `raw_bits`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::raw_bits`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        15,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `result`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::result`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        16,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `vec`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::vec`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        17,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `slice`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::slice`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        18,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `str`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::str`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        19,
                    ),
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
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `visual`,
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
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `core::visual`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        20,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `core`,
                    ),
                },
                state_after_visibility_expr: None,
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..21,
    ),
    siblings: [
        ArenaIdxRange(
            1..21,
        ),
    ],
}