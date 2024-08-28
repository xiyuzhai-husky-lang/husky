```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Err {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 3,
                },
                error: AstError::Original(
                    OriginalAstError::UnexpectedModUnderForm,
                ),
            },
            AstData::Err {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 5,
                },
                error: AstError::Original(
                    OriginalAstError::ExpectedIdentForTypeVariant(
                        TokenStreamState {
                            next_token_idx: TokenIdx(
                                15,
                            ),
                            drained: true,
                        },
                    ),
                ),
            },
            AstData::Err {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                error: AstError::Original(
                    OriginalAstError::SubmoduleFileNotFound {
                        ident_token: IdentToken {
                            ident: `submodule_name`,
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                        error: None,
                    },
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`syntax_errors::ast`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Struct,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `A`,
                    token_idx: TokenIdx(
                        4,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        5,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`syntax_errors::ast::A`, `Struct`),
                    variants: None,
                },
            },
            AstData::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                items: Some(
                    Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
                ),
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 4,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`syntax_errors::ast`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Enum,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `IllFormedEnumType`,
                    token_idx: TokenIdx(
                        13,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        14,
                    ),
                    drained: true,
                },
                block: DefnBlock::Type {
                    path: TypePath(`syntax_errors::ast::IllFormedEnumType`, `Enum`),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
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
            0..1,
        ),
        ArenaIdxRange(
            2..6,
        ),
    ],
}
```