Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
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
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            4,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`natural_number_game::Nat`, `Inductive`),
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
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            6,
                        ),
                        drained: false,
                    },
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
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
                                    value: 48,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
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
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            2,
                        ),
                        drained: true,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 55,
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
                Ast::ImplBlock {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    items: Some(
                        Type(
                            TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    2..5,
                                ),
                            },
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedIdent(
                            TokenStreamState {
                                next_token_idx: TokenIdx(
                                    44,
                                ),
                                drained: false,
                            },
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
                                    value: 48,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `OddNat`,
                        token_idx: TokenIdx(
                            83,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            84,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 56,
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
                                    value: 48,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    item_kind: MajorItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `EvenNat`,
                        token_idx: TokenIdx(
                            112,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            113,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 57,
                            },
                        ),
                        variants: None,
                    },
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            5..11,
        ),
        siblings: [
            ArenaIdxRange(
                2..5,
            ),
            ArenaIdxRange(
                5..11,
            ),
        ],
    },
)