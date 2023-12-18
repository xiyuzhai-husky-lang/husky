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
                                parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                ident: `Known`,
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
                                parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                ident: `Unknown`,
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
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `Yes`,
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
                                parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                ident: `No`,
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
                                value: 48,
                            },
                        ),
                    ),
                    variant: Protected,
                },
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedFunctionFn,
                    ),
                },
                ident_token: IdentToken {
                    ident: `default`,
                    token_idx: TokenIdx(
                        72,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        73,
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
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    14,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    15,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    16,
                ),
                body: None,
            },
            Ast::MatchStmt {
                token_group_idx: TokenGroupIdx(
                    14,
                ),
                pattern_stmt: 7,
                case_branches: ArenaIdxRange(
                    8..10,
                ),
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    12,
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
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedType,
                    ),
                },
                ident_token: IdentToken {
                    ident: `Output`,
                    token_idx: TokenIdx(
                        105,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        106,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
                    body: None,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    13,
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
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedFunctionFn,
                    ),
                },
                ident_token: IdentToken {
                    ident: `unveil`,
                    token_idx: TokenIdx(
                        111,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        112,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                10..11,
                            ),
                        },
                    ),
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    19,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ident: `ConfidentYes`,
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        192,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentYes`,
                    token_idx: TokenIdx(
                        193,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        194,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    20,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ident: `ConfidentNo`,
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        194,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentNo`,
                    token_idx: TokenIdx(
                        195,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        196,
                    ),
                    drained: true,
                },
            },
            Ast::TypeVariant {
                token_group_idx: TokenGroupIdx(
                    21,
                ),
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ident: `Unconfident`,
                            },
                        ),
                    },
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        196,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Unconfident`,
                    token_idx: TokenIdx(
                        197,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        198,
                    ),
                    drained: true,
                },
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    25,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    26,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    27,
                ),
                body: None,
            },
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    28,
                ),
                body: None,
            },
            Ast::MatchStmt {
                token_group_idx: TokenGroupIdx(
                    25,
                ),
                pattern_stmt: 16,
                case_branches: ArenaIdxRange(
                    17..20,
                ),
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    23,
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
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedType,
                    ),
                },
                ident_token: IdentToken {
                    ident: `Output`,
                    token_idx: TokenIdx(
                        225,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        226,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
                    body: None,
                },
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    24,
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
                item_kind: AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedFunctionFn,
                    ),
                },
                ident_token: IdentToken {
                    ident: `unveil`,
                    token_idx: TokenIdx(
                        231,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        232,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssociatedItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                20..21,
                            ),
                        },
                    ),
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
                                11..13,
                            ),
                        },
                    ),
                ),
            },
            Ast::Attr {
                token_group_idx: TokenGroupIdx(
                    17,
                ),
                ident: `derive`,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    18,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                177,
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
                        179,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        180,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                13..16,
                            ),
                        },
                    ),
                },
            },
            Ast::ImplBlock {
                token_group_idx: TokenGroupIdx(
                    22,
                ),
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                21..23,
                            ),
                        },
                    ),
                ),
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    29,
                ),
                visibility_expr: VisibilityExpr {
                    visibility: Pub,
                    variant: Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                302,
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
                        304,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        305,
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
        23..33,
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
            7..7,
        ),
        ArenaIdxRange(
            8..8,
        ),
        ArenaIdxRange(
            8..8,
        ),
        ArenaIdxRange(
            10..11,
        ),
        ArenaIdxRange(
            11..13,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            17..17,
        ),
        ArenaIdxRange(
            17..17,
        ),
        ArenaIdxRange(
            17..17,
        ),
        ArenaIdxRange(
            20..21,
        ),
        ArenaIdxRange(
            21..23,
        ),
        ArenaIdxRange(
            23..23,
        ),
        ArenaIdxRange(
            23..33,
        ),
    ],
}