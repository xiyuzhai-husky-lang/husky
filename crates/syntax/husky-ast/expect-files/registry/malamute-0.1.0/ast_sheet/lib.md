Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                        ident: `Within`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            15,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Within`,
                        token_idx: TokenIdx(
                            16,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            17,
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
                        ident: `Other`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            20,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Other`,
                        token_idx: TokenIdx(
                            21,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            22,
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
                            42,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Yes`,
                        token_idx: TokenIdx(
                            43,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            44,
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
                            44,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `No`,
                        token_idx: TokenIdx(
                            45,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            46,
                        ),
                        drained: true,
                    },
                },
                Ast::Defn {
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
                            68,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            69,
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
                            92,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ConfidentYes`,
                        token_idx: TokenIdx(
                            93,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            94,
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
                            94,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ConfidentNo`,
                        token_idx: TokenIdx(
                            95,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            96,
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
                            96,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Unconfident`,
                        token_idx: TokenIdx(
                            97,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            98,
                        ),
                        drained: true,
                    },
                },
                Ast::Defn {
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
                            121,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            122,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `derive`,
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
                                    9,
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
                            11,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            12,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 58,
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
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    ident: `derive`,
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
                                    31,
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
                            33,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            34,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 59,
                            },
                        ),
                        variants: Some(
                            TypeVariants {
                                ast_idx_range: ArenaIdxRange(
                                    2..4,
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
                                    4..5,
                                ),
                            },
                        ),
                    ),
                },
                Ast::Decr {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    ident: `derive`,
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
                                    81,
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
                            83,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            84,
                        ),
                        drained: false,
                    },
                    block: Type {
                        path: TypePath(
                            Id {
                                value: 60,
                            },
                        ),
                        variants: Some(
                            TypeVariants {
                                ast_idx_range: ArenaIdxRange(
                                    5..8,
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
                                    8..9,
                                ),
                            },
                        ),
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    125,
                                ),
                            },
                        },
                    },
                    item_kind: MajorItem {
                        module_item_kind: Fugitive(
                            Gn,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `narrow_down`,
                        token_idx: TokenIdx(
                            127,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            128,
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
            9..18,
        ),
        siblings: [
            ArenaIdxRange(
                4..4,
            ),
            ArenaIdxRange(
                4..5,
            ),
            ArenaIdxRange(
                8..8,
            ),
            ArenaIdxRange(
                8..9,
            ),
            ArenaIdxRange(
                9..9,
            ),
            ArenaIdxRange(
                9..18,
            ),
        ],
    },
)