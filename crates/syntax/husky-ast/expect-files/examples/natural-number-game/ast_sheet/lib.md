Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    path: TypeVariantPath {
                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                        ident: `Zero`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            2,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Zero`,
                        token_idx: TokenIdx(
                            3,
                        ),
                    },
                    state_after: TokenIdx(
                        4,
                    ),
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    path: TypeVariantPath {
                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                        ident: `Succ`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            4,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Succ`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    state_after: TokenIdx(
                        6,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Nat`,
                        token_idx: TokenIdx(
                            1,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        2,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 49,
                            },
                        ),
                        variants: Some(
                            TypeVariants {
                                ast_idx_range: ArenaIdxRange(
                                    0..2,
                                ),
                            },
                        ),
                    },
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedIdent(
                            TokenIdx(
                                46,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `OddNat`,
                        token_idx: TokenIdx(
                            85,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        86,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 50,
                            },
                        ),
                        variants: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `EvenNat`,
                        token_idx: TokenIdx(
                            114,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenIdx(
                        115,
                    ),
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 51,
                            },
                        ),
                        variants: None,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..10,
        ),
        siblings: [
            ArenaIdxRange(
                2..10,
            ),
        ],
    },
)