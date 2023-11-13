AstSheet {
    ast_arena: Arena {
        data: [
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                    ident: `Known`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        16,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Known`,
                    token_idx: TokenIdx(
                        17,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        18,
                    ),
                    drained: false,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    3,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                    ident: `Unknown`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        21,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Unknown`,
                    token_idx: TokenIdx(
                        22,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        23,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    6,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                    ident: `Yes`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        45,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Yes`,
                    token_idx: TokenIdx(
                        46,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        47,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    7,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                    ident: `No`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        47,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `No`,
                    token_idx: TokenIdx(
                        48,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        49,
                    ),
                    drained: true,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    9,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 49,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedType,
                    ),
                },
                ident_token: IdentToken {
                    ident: `Output`,
                    token_idx: TokenIdx(
                        73,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        74,
                    ),
                    drained: false,
                },
                block: AssociatedItem {
                    body: None,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    12,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    ident: `ConfidentYes`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        99,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentYes`,
                    token_idx: TokenIdx(
                        100,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        101,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    13,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    ident: `ConfidentNo`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        101,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentNo`,
                    token_idx: TokenIdx(
                        102,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        103,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    14,
                ),
                variant_path: TypeVariantPath {
                    parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    ident: `Unconfident`,
                },
                vertical_token: VerticalToken(
                    TokenIdx(
                        103,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Unconfident`,
                    token_idx: TokenIdx(
                        104,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        105,
                    ),
                    drained: true,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    16,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: PubUnder(
                        ModulePath(
                            Id {
                                value: 49,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedType,
                    ),
                },
                ident_token: IdentToken {
                    ident: `Output`,
                    token_idx: TokenIdx(
                        130,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        131,
                    ),
                    drained: false,
                },
                block: AssociatedItem {
                    body: None,
                },
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                ident: `derive`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `Class`,
                    token_idx: TokenIdx(
                        12,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        13,
                    ),
                    drained: false,
                },
                block: Type {
                    path: TypePath(
                        Id {
                            value: 61,
                        },
                    ),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                1..3,
                            ),
                        },
                    ),
                },
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    4,
                ),
                ident: `derive`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    5,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                32,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `OneVsAll`,
                    token_idx: TokenIdx(
                        34,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        35,
                    ),
                    drained: false,
                },
                block: Type {
                    path: TypePath(
                        Id {
                            value: 62,
                        },
                    ),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                3..5,
                            ),
                        },
                    ),
                },
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    8,
                ),
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                5..6,
                            ),
                        },
                    ),
                ),
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    10,
                ),
                ident: `derive`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    11,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                86,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `OneVsAllResult`,
                    token_idx: TokenIdx(
                        88,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        89,
                    ),
                    drained: false,
                },
                block: Type {
                    path: TypePath(
                        Id {
                            value: 63,
                        },
                    ),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                6..9,
                            ),
                        },
                    ),
                },
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    15,
                ),
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                9..10,
                            ),
                        },
                    ),
                ),
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    17,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                134,
                            ),
                        },
                    },
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        FunctionGn,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `narrow_down`,
                    token_idx: TokenIdx(
                        136,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        137,
                    ),
                    drained: false,
                },
                block: Fugitive {
                    path: FugitivePath(
                        Id {
                            value: 79,
                        },
                    ),
                    body: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        10..19,
    ),
    siblings: [
        ArenaIdxRange(
            5..5,
        ),
        ArenaIdxRange(
            5..6,
        ),
        ArenaIdxRange(
            9..9,
        ),
        ArenaIdxRange(
            9..10,
        ),
        ArenaIdxRange(
            10..10,
        ),
        ArenaIdxRange(
            10..19,
        ),
    ],
}