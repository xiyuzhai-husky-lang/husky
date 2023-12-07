AstSheet {
    ast_arena: Arena {
        data: [
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 214,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 523,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
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
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 214,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 211,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
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
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 217,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 271,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        47,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Yes`,
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
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    7,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 217,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 526,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        49,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `No`,
                    token_idx: TokenIdx(
                        50,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        51,
                    ),
                    drained: true,
                },
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    10,
                ),
                body: None,
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
                        MethodFn,
                    ),
                },
                ident_token: IdentToken {
                    ident: `default`,
                    token_idx: TokenIdx(
                        71,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        72,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                5..6,
                            ),
                        },
                    ),
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    12,
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
                        102,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        103,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
                    body: None,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    15,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 220,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 528,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        130,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentYes`,
                    token_idx: TokenIdx(
                        131,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        132,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    16,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 220,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 529,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        132,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentNo`,
                    token_idx: TokenIdx(
                        133,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        134,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    17,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(
                                    ItemPathId(
                                        Id {
                                            value: 220,
                                        },
                                    ),
                                ),
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 530,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        134,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Unconfident`,
                    token_idx: TokenIdx(
                        135,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        136,
                    ),
                    drained: true,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    19,
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
                        163,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        164,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
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
                block: DefnBlock::Type {
                    path: TypePath(`malamute::Class`, `Enum`),
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
                block: DefnBlock::Type {
                    path: TypePath(`malamute::OneVsAll`, `Enum`),
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
                                6..7,
                            ),
                        },
                    ),
                ),
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    11,
                ),
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                7..8,
                            ),
                        },
                    ),
                ),
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    13,
                ),
                ident: `derive`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    14,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                115,
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
                        117,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        118,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                8..11,
                            ),
                        },
                    ),
                },
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    18,
                ),
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                11..12,
                            ),
                        },
                    ),
                ),
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    20,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                167,
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
                        169,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        170,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                    body: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        12..22,
    ),
    siblings: [
        ArenaIdxRange(
            5..5,
        ),
        ArenaIdxRange(
            5..6,
        ),
        ArenaIdxRange(
            6..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..8,
        ),
        ArenaIdxRange(
            11..11,
        ),
        ArenaIdxRange(
            11..12,
        ),
        ArenaIdxRange(
            12..12,
        ),
        ArenaIdxRange(
            12..22,
        ),
    ],
}