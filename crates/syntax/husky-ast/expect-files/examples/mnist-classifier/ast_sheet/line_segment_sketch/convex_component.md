Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
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
                                    value: 40,
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
                                    value: 40,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `ConvexComponent`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        8,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 42,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    items: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..2,
                            ),
                        },
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..5,
        ),
        siblings: [
            ArenaIdxRange(
                0..2,
            ),
            ArenaIdxRange(
                2..5,
            ),
        ],
    },
)