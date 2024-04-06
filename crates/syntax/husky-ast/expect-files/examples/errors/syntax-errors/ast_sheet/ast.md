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
                    OriginalAstError::UnexpectedModUnderFugitive,
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
                        error: VfsError::FileNotExists(
                            VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../examples/errors/syntax-errors/src/ast/submodule_name.hsy",
                                ),
                            },
                        ),
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
                        `syntax_errors::ast`,
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
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..4,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..1,
        ),
        ArenaIdxRange(
            1..4,
        ),
    ],
}
```