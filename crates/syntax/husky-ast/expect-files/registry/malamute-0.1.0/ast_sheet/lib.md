Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                        ident: `Within`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            6,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Within`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            8,
                        ),
                        drained: false,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                        ident: `Other`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            11,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Other`,
                        token_idx: TokenIdx(
                            12,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            13,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                        ident: `Yes`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            24,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Yes`,
                        token_idx: TokenIdx(
                            25,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            26,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                        ident: `No`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            26,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `No`,
                        token_idx: TokenIdx(
                            27,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            28,
                        ),
                        drained: true,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
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
                            50,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            51,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        ident: `ConfidentYes`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            65,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ConfidentYes`,
                        token_idx: TokenIdx(
                            66,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            67,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        ident: `ConfidentNo`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            67,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ConfidentNo`,
                        token_idx: TokenIdx(
                            68,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            69,
                        ),
                        drained: true,
                    },
                },
                Ast::TypeVariant {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    variant_path: TypeVariantPath {
                        parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        ident: `Unconfident`,
                    },
                    vertical_token: VerticalToken(
                        TokenIdx(
                            69,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Unconfident`,
                        token_idx: TokenIdx(
                            70,
                        ),
                    },
                    state_after: TokenStreamState {
                        next_token_idx: TokenIdx(
                            71,
                        ),
                        drained: true,
                    },
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        13,
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
                            94,
                        ),
                    },
                    is_generic: false,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            95,
                        ),
                        drained: false,
                    },
                    block: AssociatedItem {
                        body: None,
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
                    item_kind: MajorItem {
                        module_item_kind: Type(
                            Enum,
                        ),
                        connection: Connected,
                    },
                    ident_token: IdentToken {
                        ident: `Class`,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    13,
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
                            15,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            16,
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
                        6,
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
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    54,
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
                            56,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            57,
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
                        12,
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
                        14,
                    ),
                    visibility_expr: VisibilityExpr {
                        visibility: Pub,
                        variant: Pub {
                            pub_token: PubToken {
                                token_idx: TokenIdx(
                                    98,
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
                            100,
                        ),
                    },
                    is_generic: true,
                    saved_stream_state: TokenStreamState {
                        next_token_idx: TokenIdx(
                            101,
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
            9..15,
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
                9..15,
            ),
        ],
    },
)