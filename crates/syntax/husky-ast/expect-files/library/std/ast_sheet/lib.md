AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `prelude`,
                    token_idx: TokenIdx(
                        2,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        3,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `std::prelude`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `logic`,
                    token_idx: TokenIdx(
                        4,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        5,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `std::logic`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: Module,
                ident_token: IdentToken {
                    ident: `ops`,
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
                                        `std::ops`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..4,
    ),
    siblings: [
        ArenaIdxRange(
            1..4,
        ),
    ],
}