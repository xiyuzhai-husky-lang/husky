```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Sorc {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Extern,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `LxAstId`,
                    token_idx: TokenIdx(
                        18,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        19,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`latex_ast_hsy::LxAstId`, `Extern`),
                    variants: None,
                },
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
                                20,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::StaticVar,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `AST`,
                    token_idx: TokenIdx(
                        23,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        24,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
                    body: None,
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        0..3,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..0,
        ),
        ArenaIdxRange(
            0..3,
        ),
    ],
}
```