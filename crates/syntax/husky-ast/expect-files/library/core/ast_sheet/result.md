Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    path: TypeVariantPath {
                        ty_path: TypePath(`core::result::Result`, `Enum`),
                        ident: `Ok`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            8,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Ok`,
                        token_idx: TokenIdx(
                            9,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            10,
                        ),
                        drained: false,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    path: TypeVariantPath {
                        ty_path: TypePath(`core::result::Result`, `Enum`),
                        ident: `Err`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            13,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Err`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            15,
                        ),
                        drained: false,
                    },
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: None,
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                        variant: Protected,
                    },
                    entity_kind: AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            AssociatedType,
                        ),
                    },
                    ident_token: IdentToken {
                        ident: `Continue`,
                        token_idx: TokenIdx(
                            42,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            43,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: PubUnder(
                            ModulePath(
                                Id {
                                    value: 17,
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
                        ident: `branch`,
                        token_idx: TokenIdx(
                            46,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            47,
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
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Enum,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Result`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            3,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 31,
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
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    3..5,
                                ),
                            },
                        ),
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            5..7,
        ),
        siblings: [
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..2,
            ),
            ArenaIdxRange(
                2..3,
            ),
            ArenaIdxRange(
                3..5,
            ),
            ArenaIdxRange(
                5..7,
            ),
        ],
    },
)