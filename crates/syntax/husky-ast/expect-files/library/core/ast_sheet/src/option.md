```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                variant_path: TypeVariantPath(`core::option::Option::Some`),
                vertical_token: VerticalToken(
                    TokenIdx(
                        29,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `Some`,
                    token_idx: TokenIdx(
                        30,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        31,
                    ),
                    drained: false,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 5,
                },
                variant_path: TypeVariantPath(`core::option::Option::None`),
                vertical_token: VerticalToken(
                    TokenIdx(
                        34,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `None`,
                    token_idx: TokenIdx(
                        35,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        36,
                    ),
                    drained: true,
                },
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`core::option`),
                    ),
                },
                state_after_visibility_expr: None,
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                state_after_visibility_expr: Some(
                    TokenStreamState {
                        next_token_idx: TokenIdx(
                            6,
                        ),
                        drained: false,
                    },
                ),
            },
            AstData::Attr {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                ident: `derive`,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                23,
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
                    ident: `Option`,
                    token_idx: TokenIdx(
                        25,
                    ),
                },
                is_generic: true,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        26,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`core::option::Option`, `Enum`),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                0..2,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        2..6,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            2..6,
        ),
    ],
}
```