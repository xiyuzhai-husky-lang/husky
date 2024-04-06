```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 2,
                        },
                    ),
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
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 3,
                        },
                    ),
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
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        7,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 5,
                        },
                    ),
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
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        8,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 6,
                        },
                    ),
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
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        11,
                    ),
                },
                body: None,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        10,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `malamute`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitForTypeItem(
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
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
                block: DefnBlock::AssocItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ),
                },
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        15,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        16,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        17,
                    ),
                },
                body: None,
            },
            AstData::MatchStmt {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        15,
                    ),
                },
                pattern_stmt: 6,
                case_branches: ArenaIdxRange(
                    7..9,
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        13,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `malamute`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitForTypeItem(
                        TraitItemKind::AssocType,
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
                block: DefnBlock::AssocItem {
                    body: None,
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        14,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `malamute`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitForTypeItem(
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
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
                block: DefnBlock::AssocItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                9..10,
                            ),
                        },
                    ),
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        20,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 8,
                        },
                    ),
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
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        21,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 9,
                        },
                    ),
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
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        22,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 10,
                        },
                    ),
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
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        26,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        27,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        28,
                    ),
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        29,
                    ),
                },
                body: None,
            },
            AstData::MatchStmt {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        26,
                    ),
                },
                pattern_stmt: 15,
                case_branches: ArenaIdxRange(
                    16..19,
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        24,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `malamute`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitForTypeItem(
                        TraitItemKind::AssocType,
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
                block: DefnBlock::AssocItem {
                    body: None,
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        25,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `malamute`,
                    ),
                },
                item_kind: EntityKind::AssocItem {
                    assoc_item_kind: AssocItemKind::TraitForTypeItem(
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
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
                block: DefnBlock::AssocItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                19..20,
                            ),
                        },
                    ),
                },
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Enum,
                    ),
                    connection: MajorItemConnectionKind::Connected,
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
                                0..2,
                            ),
                        },
                    ),
                },
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        6,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                32,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Enum,
                    ),
                    connection: MajorItemConnectionKind::Connected,
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
                                2..4,
                            ),
                        },
                    ),
                },
            },
            AstData::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        9,
                    ),
                },
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
            AstData::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        12,
                    ),
                },
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                10..12,
                            ),
                        },
                    ),
                ),
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        18,
                    ),
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        19,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                177,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Enum,
                    ),
                    connection: MajorItemConnectionKind::Connected,
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
                                12..15,
                            ),
                        },
                    ),
                },
            },
            AstData::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        23,
                    ),
                },
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                20..22,
                            ),
                        },
                    ),
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        30,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                302,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Fugitive(
                        MajorFugitiveKind::Ritchie(
                            RitchieItemKind::Gn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
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
                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                        Gn,
                    )`),
                    body: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        22..32,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            4..4,
        ),
        ArenaIdxRange(
            4..5,
        ),
        ArenaIdxRange(
            5..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            6..6,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            7..7,
        ),
        ArenaIdxRange(
            9..10,
        ),
        ArenaIdxRange(
            10..12,
        ),
        ArenaIdxRange(
            15..15,
        ),
        ArenaIdxRange(
            15..15,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            16..16,
        ),
        ArenaIdxRange(
            19..20,
        ),
        ArenaIdxRange(
            20..22,
        ),
        ArenaIdxRange(
            22..22,
        ),
        ArenaIdxRange(
            22..32,
        ),
    ],
}
```