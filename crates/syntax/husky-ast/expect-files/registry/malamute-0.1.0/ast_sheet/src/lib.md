```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
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
                        28,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Known`,
                    token_idx: TokenIdx(
                        29,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        30,
                    ),
                    drained: false,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
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
                        33,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Unknown`,
                    token_idx: TokenIdx(
                        34,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        35,
                    ),
                    drained: true,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 7,
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
                        59,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Yes`,
                    token_idx: TokenIdx(
                        60,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        61,
                    ),
                    drained: true,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 8,
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
                        61,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `No`,
                    token_idx: TokenIdx(
                        62,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        63,
                    ),
                    drained: true,
                },
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 11,
                },
                body: None,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 10,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`malamute`),
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
                        84,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        85,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
                    body: Some(
                        FormBody {
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
                    raw: 15,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 16,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 17,
                },
                body: None,
            },
            AstData::MatchStmt {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 15,
                },
                pattern_stmt: 6,
                case_branches: ArenaIdxRange(
                    7..9,
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 13,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`malamute`),
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
                        117,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        118,
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
                    raw: 14,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`malamute`),
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
                        123,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        124,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
                    body: Some(
                        FormBody {
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
                    raw: 20,
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
                        204,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentYes`,
                    token_idx: TokenIdx(
                        205,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        206,
                    ),
                    drained: true,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 21,
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
                        206,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `ConfidentNo`,
                    token_idx: TokenIdx(
                        207,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        208,
                    ),
                    drained: true,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 22,
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
                        208,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Unconfident`,
                    token_idx: TokenIdx(
                        209,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        210,
                    ),
                    drained: true,
                },
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 26,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 27,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 28,
                },
                body: None,
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 29,
                },
                body: None,
            },
            AstData::MatchStmt {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 26,
                },
                pattern_stmt: 15,
                case_branches: ArenaIdxRange(
                    16..19,
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 24,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`malamute`),
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
                        237,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        238,
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
                    raw: 25,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`malamute`),
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
                        243,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        244,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                19..20,
                            ),
                        },
                    ),
                },
            },
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                body: None,
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                22,
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
                        24,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        25,
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
                    raw: 5,
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 6,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                44,
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
                        46,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        47,
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
                    raw: 9,
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
                    raw: 12,
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
                    raw: 18,
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 19,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                189,
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
                        191,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        192,
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
                    raw: 23,
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
            AstData::Err {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 30,
                },
                error: AstError::Original(
                    OriginalAstError::ExpectedLboxOrIdentAfterPoundForAttrOrSorc(
                        TokenVerseIdx {
                            lcurl: None,
                            raw: 30,
                        },
                    ),
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 31,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                315,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Ritchie(
                            RitchieItemKind::Gn,
                        ),
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `narrow_down`,
                    token_idx: TokenIdx(
                        317,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        318,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: FormPath(`malamute::narrow_down`, `Ritchie(
                        Gn,
                    )`),
                    body: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        22..34,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..0,
        ),
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
            22..34,
        ),
    ],
}
```